#!/usr/bin/env bash
#
# Patch in a Safecoin v1.12 monorepo that supports 2x transactions for testing the
# SPL Token 2022 Confidential Transfer extension
#

set -e

here="$(dirname "$0")"
cd "$here"

if [[ ! -d twoxtx-safecoin ]]; then
  if [[ -n $CI ]]; then
    git config --global user.email "you@example.com"
    git config --global user.name "Your Name"
    git clone https://github.com/fair-exchange/safecoin.git twoxtx-solana
  else
    git clone git@github.com:fair-exchange/safecoin.git twoxtx-solana
  fi
fi

if [[ ! -f twoxtx-solana/.twoxtx-patched ]]; then
  git -C twoxtx-safecoin am -3 "$PWD"/twoxtx.patch
  touch twoxtx-solana/.twoxtx-patched
fi

../patch.crates-io.sh twoxtx-solana
exit 0
