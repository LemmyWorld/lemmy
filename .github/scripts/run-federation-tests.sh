#!/bin/bash
set -euxo pipefail
apt update && apt install -y bash curl postgresql-client
bash api_tests/prepare-drone-federation-test.sh
cd api_tests/
yarn
yarn api-test
