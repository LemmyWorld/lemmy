#!/bin/bash
set -euxo pipefail
cargo +nightly fmt -- --check
