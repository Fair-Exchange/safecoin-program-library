#!/usr/bin/env bash

set -e
cd "$(dirname "$0")/.."
source ./ci/safecoin-version.sh install

set -x
cd memo/ts
yarn
yarn build
yarn lint
yarn test
