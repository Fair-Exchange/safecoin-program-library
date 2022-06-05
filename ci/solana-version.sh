#
# This file maintains the safecoin versions for use by CI.
#
# Obtain the environment variables without any automatic updating:
#   $ source ci/safecoin-version.sh
#
# Obtain the environment variables and install update:
#   $ source ci/safecoin-version.sh install

# Then to access the safecoin version:
#   $ echo "$solana_version"
#

if [[ -n $SAFECOIN_VERSION ]]; then
  solana_version="$SAFECOIN_VERSION"
else
  solana_version=v1.8.0
fi

export solana_version="$solana_version"
export PATH="$HOME"/.local/share/solana/install/active_release/bin:"$PATH"

if [[ -n $1 ]]; then
  case $1 in
  install)
    sh -c "$(curl -sSfL https://release.solana.com/$solana_version/install)"
    safecoin --version
    ;;
  *)
    echo "$0: Note: ignoring unknown argument: $1" >&2
    ;;
  esac
fi
