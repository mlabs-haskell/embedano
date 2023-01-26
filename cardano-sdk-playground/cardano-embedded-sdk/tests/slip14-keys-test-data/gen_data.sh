#!/usr/bin/env bash


set -euo pipefail

ROOT_KEY=$(./cardano-address key from-recovery-phrase Shelley < slip14.mnemonic)
./bech32 <<< ${ROOT_KEY} > root_key_hex

ADDR_0_XPRV=$(./cardano-address key child 1852H/1815H/0H/0/0 <<< ${ROOT_KEY})      
./bech32 <<< ${ADDR_0_XPRV} > addr_0_xprv_hex
# echo $ADDR_0_XPRV

ADDR_0_XPUB=$(./cardano-address key public --with-chain-code <<< ${ADDR_0_XPRV})
./bech32 <<< ${ADDR_0_XPUB} > addr_0_xpub_hex
