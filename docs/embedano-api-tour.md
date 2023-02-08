# Embedano SDK API

## Main SDK functions

Main SDK functions are designed for easy integration with embedded firmware. They require basic parameters:

- entropy or seed of the wallet, that usually stored in device memory
- password, that usually provided by the user
- derivation path to determine which keys of HD wallet to use (see [HD Sequential wallets (à la BIP-44)](https://input-output-hk.github.io/cardano-wallet/concepts/address-derivation))

Main functions allow to:

- sign transaction id (hash of transaction body)
- sing arbitrary data
- derive private and public keys from known seed
- check that particular public key belongs to HD wallet (that it can be derived from current seed, to be precise)

## TODO

- one rundown for all functions in api.rs (test via main.rs)
- refine test cases so they can be included in report
