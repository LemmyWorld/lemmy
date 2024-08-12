#!/bin/sh
set -euxo pipefail
apk add git
IGNORED=$(git ls-files --cached -i --exclude-standard)
if [[ "$IGNORED" ]]; then echo "Ignored files present:\n$IGNORED\n"; exit 1; fi
