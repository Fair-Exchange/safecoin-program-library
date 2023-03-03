#!/usr/bin/env bash

set -ex
cd "$(dirname "$0")/.."
source ./ci/safecoin-version.sh install

cd token-swap/js
npm install
npm run lint
npm run build
npm run start-with-test-validator
(cd ../../target/deploy && mv safe_token_swap_production.so safe_token_swap.so)
SWAP_PROGRAM_OWNER_FEE_ADDRESS="HfoTxFR1Tm6kGmWgYWD6J7YHVy1UwqSULUGVLXkJqaKN" npm run start-with-test-validator
