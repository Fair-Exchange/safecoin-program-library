#!/usr/bin/env bash
#
# Patches the SPL crates for developing against a local safecoin monorepo
#

solana_dir=$1
if [[ -z $solana_dir ]]; then
  echo "Usage: $0 <path-to-solana-monorepo>"
  exit 1
fi

workspace_crates=(
  Cargo.toml
  themis/client_ristretto/Cargo.toml
)

if [[ ! -r "$solana_dir"/scripts/read-cargo-variable.sh ]]; then
  echo "$solana_dir is not a path to the safecoin monorepo"
  exit 1
fi

set -e

solana_dir=$(cd "$solana_dir" && pwd)
cd "$(dirname "$0")"

source "$solana_dir"/scripts/read-cargo-variable.sh
solana_ver=$(readCargoVariable version "$solana_dir"/sdk/Cargo.toml)

echo "Patching in $solana_ver from $solana_dir"
echo
for crate in "${workspace_crates[@]}"; do
  if grep -q '\[patch.crates-io\]' "$crate"; then
    echo "$crate is already patched"
  else
    cat >> "$crate" <<PATCH
[patch.crates-io]
safecoin-account-decoder = {path = "$solana_dir/account-decoder" }
safecoin-banks-client = { path = "$solana_dir/banks-client"}
safecoin-banks-server = { path = "$solana_dir/banks-server"}
safecoin-bpf-loader-program = { path = "$solana_dir/programs/bpf_loader" }
safecoin-clap-utils = {path = "$solana_dir/clap-utils" }
safecoin-cli-config = {path = "$solana_dir/cli-config" }
safecoin-cli-output = {path = "$solana_dir/cli-output" }
safecoin-client = { path = "$solana_dir/client"}
safecoin-core = { path = "$solana_dir/core"}
safecoin-logger = {path = "$solana_dir/logger" }
safecoin-notifier = { path = "$solana_dir/notifier" }
safecoin-remote-wallet = {path = "$solana_dir/remote-wallet" }
safecoin-program = { path = "$solana_dir/sdk/program" }
safecoin-program-test = { path = "$solana_dir/program-test" }
safecoin-runtime = { path = "$solana_dir/runtime" }
safecoin-sdk = { path = "$solana_dir/sdk" }
safecoin-stake-program = { path = "$solana_dir/programs/stake" }
safecoin-transaction-status = { path = "$solana_dir/transaction-status" }
safecoin-vote-program = { path = "$solana_dir/programs/vote" }
PATCH
  fi
done

./update-solana-dependencies.sh "$solana_ver"
