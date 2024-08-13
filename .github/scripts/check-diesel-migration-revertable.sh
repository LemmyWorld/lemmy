#!/bin/bash
set -euxo pipefail
diesel migration run
diesel migration redo
