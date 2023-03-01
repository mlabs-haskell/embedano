#!/usr/bin/env bash

# Step by step keys deriation.
# `cardano-address` only accept paths of 2, 3 or 4 indexes.

set -euo pipefail

 cargo run -p demo-client -- \
  --mnemonics "all all all all all all all all all all all all" \
  --password "" \
  --wallet-address "addr1vxq0nckg3ekgzuqg7w5p9mvgnd9ym28qh5grlph8xd2z92su77c6m" \
  --script-address "addr1w9nlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4slcd85z" \
  --derivation-path "m/1852'/1815'/0'/0/0" \
  --network-id 0 \
  --node-socket "/home/mike/dev/mlabs/embedano-project/plutip-made-keys/pool-1/node.socket"