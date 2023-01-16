# SDK API (draft)

SDK API should allow us to reach 3 main goals - we should be able to:

- sign simple Cardano transaction
- proof that Payment key belongs to particular HD wallet
- sign arbitrary data

For all 3 cases we will need to be able to derive payment key for specific derivation path - probably, derivation of payment key by path may (or must?) be part of the public SDK API also.

Some questions and considerations below.

## Sign simple Cardano transaction

We will need to sign transaction body. Probably, the most general user-facing API for that could be something like

```rust
fn sign_tx_body(key_path: KeyPath, tx_id: TxId) -> Result<Witness, SomeError>
```

where

```
KeyPath - TDB
TxId - transaction body hash

Witness = (Signature, PaymentKey)
or maybe
Witness = (Signature, PaymentKey, KeyPath)
```

*Question:* how are we going to return key and signature of `Witness`? Hex encoded CBOR?

## Proof that Payment key belongs to particular HD wallet

```rust
fn proof_ownership(payment_key: PaymentKey) -> Result<Proof, SomeError>
```

Proof of key ownership will require `Account Discovery` (see links below). Maybe we can also add more specific variant where user will be able to check ownership for exact derivation path

```rust
fn confirm_ownership(key_path: KeyPath, payment_key: PaymentKey) -> Result<Proof, SomeError>
```

*Question:* how to pass Payment Key - hex encoded CBOR?

*Question:* how do we provide proof, i.e. what will be the return type of proving function? Encoded nonce or signature that can be verified with payment key?

## Signing arbitrary data

```rust
fn sign_data(key_path: KeyPath, data: Payload) -> Result<Witness, SomeError>
```
where `Payload` is some binary data.

*Question:* do we need to support some other types of `data` like hex encoded bytes?

## Derive payment key

```rust
fn derive_key(key_path: KeyPath) -> Result<PaymentKey, SomeError>
```

## Links

- [Account Discovery](https://input-output-hk.github.io/cardano-wallet/concepts/hierarchical-deterministic-wallets)
- Signatures
  - [CIP-0049](https://developers.cardano.org/docs/governance/cardano-improvement-proposals/cip-0049/)
  - [Cardano.Crypto.Signing.Signature](https://input-output-hk.github.io/ouroboros-network/cardano-crypto-wrapper/Cardano-Crypto-Signing-Signature.html)