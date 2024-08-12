#!/bin/bash
set -euxo pipefail
wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
cp cargo-binstall /usr/local/cargo/bin
cargo binstall -y cargo-machete
cargo machete
