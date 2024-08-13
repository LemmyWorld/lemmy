#!/bin/sh
set -euxo pipefail
diesel migration run
diesel migration redo
