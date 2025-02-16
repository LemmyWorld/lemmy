use crate::{
  activities::{generate_activity_id, send_lemmy_activity, verify_person},
  insert_received_activity,
  objects::{person::ApubPerson, private_message::ApubPrivateMessage},
  protocol::activities::{
    create_or_update::private_message::CreateOrUpdatePrivateMessage,
    CreateOrUpdateType,
  },
};
use activitypub_federation::{
  config::Data,
  protocol::verification::{verify_domains_match, verify_urls_match},
  traits::{ActivityHandler, Actor, Object},
};
use lemmy_api_common::context::LemmyContext;
use lemmy_db_schema::source::activity::ActivitySendTargets;
use lemmy_db_views::structs::PrivateMessageView;
use lemmy_utils::error::{LemmyError, LemmyResult};
use url::Url;

pub(crate) async fn send_create_or_update_pm(
  pm_view: PrivateMessageView,
  kind: CreateOrUpdateType,
  context: Data<LemmyContext>,
) -> LemmyResult<()> {
  let actor: ApubPerson = pm_view.creator.into();
  let recipient: ApubPerson = pm_view.recipient.into();

  let id = generate_activity_id(
    kind.clone(),
    &context.settings().get_protocol_and_hostname(),
  )?;
  let create_or_update = CreateOrUpdatePrivateMessage {
    id: id.clone(),
    actor: actor.id().into(),
    to: [recipient.id().into()],
    object: ApubPrivateMessage(pm_view.private_message.clone())
      .into_json(&context)
      .await?,
    kind,
  };
  let inbox = ActivitySendTargets::to_inbox(recipient.shared_inbox_or_inbox());
  send_lemmy_activity(&context, create_or_update, &actor, inbox, true).await
}

#[async_trait::async_trait]
impl ActivityHandler for CreateOrUpdatePrivateMessage {
  type DataType = LemmyContext;
  type Error = LemmyError;

  fn id(&self) -> &Url {
    &self.id
  }

  fn actor(&self) -> &Url {
    self.actor.inner()
  }

  async fn verify(&self, context: &Data<Self::DataType>) -> LemmyResult<()> {
    verify_person(&self.actor, context).await?;
    verify_domains_match(self.actor.inner(), self.object.id.inner())?;
    verify_domains_match(self.to[0].inner(), self.object.to[0].inner())?;
    verify_urls_match(self.actor.inner(), self.object.attributed_to.inner())?;
    ApubPrivateMessage::verify(&self.object, self.actor.inner(), context).await?;
    Ok(())
  }

  async fn receive(self, context: &Data<Self::DataType>) -> LemmyResult<()> {
    insert_received_activity(&self.id, context).await?;
    ApubPrivateMessage::from_json(self.object, context).await?;
    Ok(())
  }
}
