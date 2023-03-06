#!/usr/bin/env bash
#
# Updates the safecoin version in all the SPL crates
#

solana_ver=$1
if [[ -z $solana_ver ]]; then
  echo "Usage: $0 <new-safecoin-version>"
  exit 1
fi

cd "$(dirname "$0")"
source ./ci/safecoin-version.sh
old_solana_ver=${safecoin_version#v}

sed -i'' -e "s#safecoin_version=v.*#safecoin_version=v${solana_ver}#" ./ci/safecoin-version.sh
sed -i'' -e "s#safecoin_version = \".*\"#safecoin_version = \"${solana_ver}\"#" ./Anchor.toml

declare tomls=()
while IFS='' read -r line; do tomls+=("$line"); done < <(find . -name Cargo.toml)

crates=(
  safecoin-account-decoder
  safecoin-banks-client
  safecoin-banks-server
  safecoin-bpf-loader-program
  safecoin-clap-utils
  safecoin-clap-v3-utils
  safecoin-cli-config
  safecoin-cli-output
  safecoin-client
  safecoin-core
  safecoin-logger
  safecoin-notifier
  safecoin-program
  safecoin-program-test
  safecoin-remote-wallet
  safecoin-runtime
  safecoin-sdk
  safecoin-stake-program
  safecoin-test-validator
  safecoin-transaction-status
  safecoin-vote-program
  safecoin-version
  safe-zk-token-sdk
)

set -x
for crate in "${crates[@]}"; do
  sed -E -i'' -e "s:(${crate} = \")(=?)${old_solana_ver}\".*:\1\2${solana_ver}\":" "${tomls[@]}"
  sed -E -i'' -e "s:(${crate} = \{ version = \")(=?)${old_solana_ver}(\".*):\1\2${solana_ver}\3:" "${tomls[@]}"
done
