#!/usr/bin/env bash

# Generate script for Embedano demo
# Different nonce will lead to different script -> different address
# E.g.: preprod:  `./gen_script_and_address.sh "--testnet-magic 1"`  
#        preview: `./gen_script_and_address.sh "--testnet-magic 2"`  
#        mainnet: `./gen_script_and_address.sh "--mainnet"`  

set -euox pipefail


OUT_FILE=./demo-client/cardano-data/script.plutus

nix run github:mlabs-haskell/plutip/embedano-testnet#plutip:exe:generate-script -- \
  --nonce embedano0 \
  --out-file ${OUT_FILE}

cardano-cli address build --payment-script-file ${OUT_FILE} $1

# embedano0
#      addr1wyzfvnszsq4kx82eec4smazdr8t909gp4amwt5s86z6tqjgvsyue2%
# addr_test1wqzfvnszsq4kx82eec4smazdr8t909gp4amwt5s86z6tqjghcsqk0%
# addr_test1wqzfvnszsq4kx82eec4smazdr8t909gp4amwt5s86z6tqjghcsqk0%    