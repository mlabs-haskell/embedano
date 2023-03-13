#!/usr/bin/env bash

# Generate script for Embedano demo
# Different nonce will lead to different script -> different address
# E.g.: preprod: `./gen_script_and_address.sh "--testnet-magic 1"`  
#       preview: `./gen_script_and_address.sh "--testnet-magic 2"`  
#       mainnet: `./gen_script_and_address.sh "--mainnet"`  

set -euox pipefail


OUT_FILE=./nrf52-demo/demo-client/cardano-data/script.plutus

nix run github:mlabs-haskell/plutip/embedano-testnet#plutip:exe:generate-script -- \
  --nonce embedano_device_001 \
  --out-file ${OUT_FILE}

cardano-cli address build --payment-script-file ${OUT_FILE} $1
