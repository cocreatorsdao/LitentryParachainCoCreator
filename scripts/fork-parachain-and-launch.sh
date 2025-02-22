#!/bin/bash

set -eo pipefail

# this script:
# - scrapes the state of a given parachain using `fork-off-substrate`
# - save the state snapshot to a chain spec JSON
# - use this chain spec to launch a local parachain network

ROOTDIR=$(git rev-parse --show-toplevel)
TMPDIR=$(mktemp -d /tmp/XXXXXX)

FORK_OFF_SUBSTRATE_REPO="https://github.com/litentry/fork-off-substrate.git"

function print_divider() {
  echo "------------------------------------------------------------"
}

function usage() {
  print_divider
  echo "Usage: $0 [http-rpc-endpoint] [orig-chain] [fork-chain] [binary]"
  echo 
  echo "the http-rpc-endpoint has to be a reachabale HTTP-RPC URL (do not mix it up with ws port)"
  echo 
  echo "default:"
  echo "http-rpc-endpoint: http://localhost:9933"
  echo "orig-chain:        litmus"
  echo "fork-chain:        litmus-dev"
  echo "binary:            the binary copied from litentry/litentry-parachain:latest"
  print_divider
}

[ $# -gt 4 ] && (usage; exit 1)

case "$1" in
  help|-h|--help)
    usage
    exit 1
    ;;
  *)
    ;;
esac

ENDPOINT="${1:-http://localhost:9933}"
ORIG_CHAIN=${2:-litmus}
FORK_CHAIN=${3:-litmus-dev}
CHAIN_TYPE=

case "$FORK_CHAIN" in
  litmus*)
    CHAIN_TYPE=litmus ;;
  litentry*)
    CHAIN_TYPE=litentry ;;
  *)
    echo "unsupported chain type"
    exit 1 ;;
esac

echo "TMPDIR is $TMPDIR"
cd "$TMPDIR"
git clone "$FORK_OFF_SUBSTRATE_REPO"
cd fork-off-substrate
npm i

mkdir data && cd data

# copy the binary
if [ -z "$4" ]; then
  docker cp $(docker create --rm litentry/litentry-parachain:latest):/usr/local/bin/litentry-collator binary
else
  cp "$4" binary
fi
chmod a+x binary

# pop up a warning if on non-CI host
if [ $(hostname) != "ubuntu-16gb-CI" ]; then
  echo "WARNING: it seems you are not on the CI host"
  echo "         please make sure the given HTTP-RPC endpoint accessible"
else
  # open the ssh port forwarding for a short time
  ssh -f -L 9900:localhost:9933 litmus-sg-rpc0 sleep 120
fi

# retrieve the live wasm
curl -s -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "state_getStorage", "params": [ "0x3a636f6465" ]}' "$ENDPOINT" | \
jq .result | sed 's/"//g;s/^0x//' | xxd -r -p > runtime.wasm

# write .env file
cd ..
cat << EOF > .env
HTTP_RPC_ENDPOINT=$ENDPOINT
ALICE=1
ORIG_CHAIN=$ORIG_CHAIN
FORK_CHAIN=$FORK_CHAIN
EOF

npm start

if [ ! -f data/fork.json ]; then
  echo "cannot find data/fork.json, please check it manually"
  exit 2
fi

FORK_JSON_PATH="$(pwd)/data/fork.json"

cd "$ROOTDIR"
sed -i.bak "s;$FORK_CHAIN;$FORK_JSON_PATH;" docker/$CHAIN_TYPE-parachain-launch-config.yml

# start the network
make launch-docker-$CHAIN_TYPE
