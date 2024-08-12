#!/bin/bash
set -euxo pipefail
rustup component add rustfmt
cargo +nightly fmt -- --check
