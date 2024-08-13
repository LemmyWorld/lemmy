#!/bin/bash
set -euxo pipefail
diesel migration run
diesel print-schema --config-file=diesel.toml > tmp.schema
diff tmp.schema crates/db_schema/src/schema.rs
