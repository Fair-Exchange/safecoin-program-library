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
)

if [[ ! -r "$solana_dir"/scripts/read-cargo-variable.sh ]]; then
  echo "$solana_dir is not a path to the safecoin monorepo"
  exit 1
fi

set -e

solana_dir=$(cd "$solana_dir" && pwd)
cd "$(dirname "$0")"

source "$solana_dir"/scripts/read-cargo-variable.sh
# get version from Cargo.toml first. if it is empty, get it from other places.
solana_ver="$(readCargoVariable version "$solana_dir"/Cargo.toml)"
solana_ver=${solana_ver:-$(readCargoVariable version "$solana_dir"/sdk/Cargo.toml)}

crates_map=()
crates_map+=("safecoin-account-decoder account-decoder")
crates_map+=("safecoin-banks-client banks-client")
crates_map+=("safecoin-banks-server banks-server")
crates_map+=("safecoin-bpf-loader-program programs/bpf_loader")
crates_map+=("safecoin-clap-utils clap-utils")
crates_map+=("safecoin-clap-v3-utils clap-v3-utils")
crates_map+=("safecoin-cli-config cli-config")
crates_map+=("safecoin-cli-output cli-output")
crates_map+=("safecoin-client client")
crates_map+=("safecoin-core core")
crates_map+=("safecoin-logger logger")
crates_map+=("safecoin-notifier notifier")
crates_map+=("safecoin-remote-wallet remote-wallet")
crates_map+=("safecoin-program sdk/program")
crates_map+=("safecoin-program-test program-test")
crates_map+=("safecoin-runtime runtime")
crates_map+=("safecoin-sdk sdk")
crates_map+=("safecoin-stake-program programs/stake")
crates_map+=("safecoin-test-validator test-validator")
crates_map+=("safecoin-transaction-status transaction-status")
crates_map+=("safecoin-version version")
crates_map+=("safecoin-vote-program programs/vote")
crates_map+=("safe-zk-token-sdk zk-token-sdk")

patch_crates=()
for map_entry in "${crates_map[@]}"; do
  read -r crate_name crate_path <<<"$map_entry"
  full_path="$solana_dir/$crate_path"
  if [[ -r "$full_path/Cargo.toml" ]]; then
    patch_crates+=("$crate_name = { path = \"$full_path\" }")
  fi
done

echo "Patching in $solana_ver from $solana_dir"
echo
for crate in "${workspace_crates[@]}"; do
  if grep -q '\[patch.crates-io\]' "$crate"; then
    echo "$crate is already patched"
  else
    cat >> "$crate" <<PATCH
[patch.crates-io]
$(printf "%s\n" "${patch_crates[@]}")
PATCH
  fi
done

./update-solana-dependencies.sh "$solana_ver"
