#
# This file maintains the safecoin versions for use by CI.
#
# Obtain the environment variables without any automatic updating:
#   $ source ci/safecoin-version.sh
#
# Obtain the environment variables and install update:
#   $ source ci/safecoin-version.sh install

# Then to access the safecoin version:
#   $ echo "$safecoin_version"
#

if [[ -n $SAFECOIN_VERSION ]]; then
  safecoin_version="$SAFECOIN_VERSION"
else
  safecoin_version=v1.14.17
fi

export safecoin_version="$safecoin_version"
export PATH="$HOME"/.local/share/solana/install/active_release/bin:"$PATH"

if [[ -n $1 ]]; then
  case $1 in
  install)
    sh -c "$(curl -sSfL https://release.solana.com/$safecoin_version/install)"
    safecoin --version
    ;;
  *)
    echo "safecoin-version.sh: Note: ignoring unknown argument: $1" >&2
    ;;
  esac
fi
