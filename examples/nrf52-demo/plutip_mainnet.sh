#!/usr/bin/env bash

# Run Plutip local cluster with mainnet.
# Keys will be stored in work-dir # todo: there was some more convinient copy mechinism

set -euox pipefail



nix run github:mlabs-haskell/plutip/embedano-testnet#plutip:exe:local-cluster --  \
  --no-index \
  --working-dir /home/mike/dev/mlabs/embedano-project/plutip-made-keys \
  --mnemonics "$(cat ./demo-client/keys/mnemonics)"