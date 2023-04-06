#!/usr/bin/env bash

# Demo runner

set -euo pipefail

 cargo run -p demo-client -- \
  --mnemonics "initial label sand movie check train leaf escape hurt sort remove risk" \
  --password "" \
  --derivation-path "m/1852'/1815'/0'/0/0" \
  --script-address "addr_test1wr5qpejpzx7szat38a58v246jk6hmexcvnfza5nsdvperqgvjcfxd" \
  --network preprod \
  --node-socket "./cardano-node.socket" \
  --device-port $1 \
  --mode $2