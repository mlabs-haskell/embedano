#!/usr/bin/env bash

# Step by step keys deriation.
# `cardano-address` only accept paths of 2, 3 or 4 indexes.

set -euo pipefail

echo all all all all all all all all all all all all > phrase.prv
./cardano-address key from-recovery-phrase Shelley < phrase.prv > root.xsk

echo -e "Mnemonic"
cat phrase.prv
echo -e "\nRoot"
./cardano-address key inspect <<< $(cat root.xsk) 


path=1852H/1815H
echo -e "\nDerived for ${path}"
derived=$(./cardano-address key child ${path} < root.xsk)                       
./cardano-address key inspect <<< $derived

path=1852H/1815H/0H
echo -e "\nDerived for ${path}"
derived=$(./cardano-address key child ${path} < root.xsk)                       
./cardano-address key inspect <<< $derived

path=1852H/1815H/0H/0/0
echo -e "\nDerived for ${path}"
derived=$(./cardano-address key child ${path} < root.xsk)                       
./cardano-address key inspect <<< $derived

echo -e "\nPubKey for ${path}"
pub=$(./cardano-address key public --with-chain-code <<< ${derived})
./cardano-address key inspect <<< $pub
