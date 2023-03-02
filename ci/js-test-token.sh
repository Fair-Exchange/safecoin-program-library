#!/usr/bin/env bash

set -e
cd "$(dirname "$0")/.."
source ./ci/safecoin-version.sh install

set -x
cd token/js

npm install
npm run lint
npm run build
npm test
