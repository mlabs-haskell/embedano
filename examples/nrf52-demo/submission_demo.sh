#!/usr/bin/env bash

# Step by step keys deriation.
# `cardano-address` only accept paths of 2, 3 or 4 indexes.

set -euo pipefail

 cargo run -p demo-client -- \
  --mnemonics "initial label sand movie check train leaf escape hurt sort remove risk" \
  --password "" \
  --derivation-path "m/1852'/1815'/0'/0/0" \
  --script-address "addr_test1wph0tef2u7jdhdpvkyf6fsvt8dc0m5u70ngfwwzxzstcfeq0j3v8e" \
  --network preprod \
  --node-socket "./cardano-node.socket"