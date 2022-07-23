#!/usr/bin/env bash

set -ex
cd "$(dirname "$0")/.."
source ./ci/safecoin-version.sh install

cd stake-pool/py
python3 -m venv venv
source ./venv/bin/activate
pip3 install -r requirements.txt
check_dirs=(
  "bot"
  "safe_token"
  "stake"
  "stake_pool"
  "system"
  "tests"
  "vote"
)
flake8 "${check_dirs[@]}"
mypy "${check_dirs[@]}"
python3 -m pytest
