#!/usr/bin/env bash

# Step by step keys deriation.
# `cardano-address` only accept paths of 2, 3 or 4 indexes.

set -euo pipefail

 cargo run -p demo-client -- \
  --mnemonics "initial label sand movie check train leaf escape hurt sort remove risk" \
  --password "" \
  --derivation-path "m/1852'/1815'/0'/0/0" \
  --script-address "addr_test1wq3605f486dnjtu77cha4w5v27t3cs6drr35esjjeu7m5acrfwwrn" \
  --network preprod \
  --node-socket "./cardano-node.socket"