#!/usr/bin/env bash

# Step by step keys deriation.
# `cardano-address` only accept paths of 2, 3 or 4 indexes.

set -euo pipefail

 cargo run -p demo-client -- \
  --mnemonics "initial label sand movie check train leaf escape hurt sort remove risk" \
  --password "" \
  --wallet-address "addr1v88mveycz7jftzq7ljql066ygpqh4arrq0j7kkwyx95guvq9vqyz7" \
  --script-address "addr1w9nlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4slcd85z" \
  --derivation-path "m/1852'/1815'/0'/0/0" \
  --network-id 0 \
  --node-socket "/home/mike/dev/mlabs/embedano-project/plutip-made-keys/pool-1/node.socket"