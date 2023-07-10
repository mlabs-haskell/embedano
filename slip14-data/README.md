# SLIP-0014 HD wallets test data

- [SLIP-0014 HD wallets test data](#slip-0014-hd-wallets-test-data)
  - [SLIP-0014](#slip-0014)
  - [Keys](#keys)
    - [Root extended signing key](#root-extended-signing-key)
    - [Extended signing key for address 0](#extended-signing-key-for-address-0)
    - [Verification key for address 0](#verification-key-for-address-0)
      - [Key hash](#key-hash)
    - [Keys derivation steps](#keys-derivation-steps)
  - [Simple transaction](#simple-transaction)
    - [Transaction id (body hash)](#transaction-id-body-hash)
    - [Transaction body (a.k.a. raw transaction)](#transaction-body-aka-raw-transaction)
    - [Signed transaction](#signed-transaction)
    - [Transaction views](#transaction-views)
      - [Body](#body)
      - [Singed](#singed)
      - [Witness](#witness)

## SLIP-0014

See [slip-0014 HD wallet](https://github.com/satoshilabs/slips/blob/master/slip-0014.md)

## Keys

Keys were generated with `cardano-address` and `cardano-wallet` Haskell libraries.

### Root extended signing key

[Link to file](keys/root-extended-key.skey)

Generated from mnemonic phrase

```shell
all all all all all all all all all all all all
```

```shell
{
    "type": "PaymentExtendedSigningKeyShelley_ed25519_bip32",
    "description": "Root Extended Key",
    "cborHex": "588078fe04891cbda885b3ee9b7a60bb5991c3209b07f16324c2d68cb9c7c328ed512a18cdf9b5c0fa98e7d620ae9d851a58aca7e4e0ab46f607c03e78498b345b1b80def65319d69eb65c59d6a67b18b27f03c9c005f5499f75bdb8ac5ba4b5104b7c0b5c44c1ddb9049bfcaf4ec5d73236392321c69979bbcff1f7c1b6d74c9c5a"
}
```

### Extended signing key for address 0

[Link to file](keys/address-0-signing-key.skey)

Derived from root key by path `1852'/1815'/0'/0/0`

```shell
{
    "type": "PaymentExtendedSigningKeyShelley_ed25519_bip32",
    "description": "Payment Signing Key",
    "cborHex": "5880286821b9f84458fe1644723e9f0bc6ee75d71efff96512168a3f62cbdc28ed519e5976fcff6ac1801e04ca66051584abbcef38f16a411763af8c3dc14a9727d55d010cf16fdeff40955633d6c565f3844a288a24967cf6b76acbeb271b4f13c1f123474e140a2c360b01f0fa66f2f22e2e965a5b07a80358cf75f77abbd66088"
}
```

### Verification key for address 0

[Link to file](keys/address-0-verification-key.vkey)

```shell
{
    "type": "PaymentVerificationKeyShelley_ed25519",
    "description": "Payment Verification Key",
    "cborHex": "58205d010cf16fdeff40955633d6c565f3844a288a24967cf6b76acbeb271b4f13c1"
}
```

#### Key hash

```shell
80f9e2c88e6c817008f3a812ed889b4a4da8e0bd103f86e7335422aa
```

### Keys derivation steps

Derivations made with `cardano-address`. `cardano-address` only accept paths of 2, 3 or 4 indexes.

[Script](keys_derivation_steps.sh)

```shell
Mnemonic
all all all all all all all all all all all all

Root
{
    "chain_code": "7c0b5c44c1ddb9049bfcaf4ec5d73236392321c69979bbcff1f7c1b6d74c9c5a",
    "key_type": "private",
    "extended_key": "78fe04891cbda885b3ee9b7a60bb5991c3209b07f16324c2d68cb9c7c328ed512a18cdf9b5c0fa98e7d620ae9d851a58aca7e4e0ab46f607c03e78498b345b1b"
}

Derived for 1852H/1815H
{
    "chain_code": "4d42bb8c2498a43c64df25fa2e316ec8a315b3a6f391dfb56475c7db59aec338",
    "key_type": "private",
    "extended_key": "3ec88c70025e361df240cbbd919a1cf34231749842fdccbabfdde278dc11ae0282bad54d095c2724445136150f7236157fb8f61e3f0c24151df05f56a55dd7c4"
}

Derived for 1852H/1815H/0H
{
    "chain_code": "b19657ad13ee581b56b0f8d744d66ca356b93d42fe176b3de007d53e9c4c4e7a",
    "key_type": "private",
    "extended_key": "1809183f2042b48a70409a99067ba017f8f81967a0e0431dec5157fcd328ed51cc4a7129f432a4b68095036df22fef5b5d10e96583828df0cd44bda8b5a172b0"
}

Derived for 1852H/1815H/0H/0/0
{
    "chain_code": "f123474e140a2c360b01f0fa66f2f22e2e965a5b07a80358cf75f77abbd66088",
    "key_type": "private",
    "extended_key": "286821b9f84458fe1644723e9f0bc6ee75d71efff96512168a3f62cbdc28ed519e5976fcff6ac1801e04ca66051584abbcef38f16a411763af8c3dc14a9727d5"
}

PubKey for 1852H/1815H/0H/0/0
{
    "chain_code": "f123474e140a2c360b01f0fa66f2f22e2e965a5b07a80358cf75f77abbd66088",
    "key_type": "public",
    "extended_key": "5d010cf16fdeff40955633d6c565f3844a288a24967cf6b76acbeb271b4f13c1"
}
```

## Simple transaction

Simple transaction that pays 111 Ada from slip-0014 wallet address 0 to some other address on `Mainnet`.

Transaction was generated and signed with `cardano-cli` v 1.35.3

### Transaction id (body hash)

 ```shell
bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb

```

### Transaction body (a.k.a. raw transaction)

[Link to file](txs/tx-bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb.raw)

```shell
{
    "type": "TxBodyBabbage",
    "description": "",
    "cborHex": "86a40081825820fb03abe73ddca76bc2f4a4fd18fde3b8e7844d7d1e3049042b4ed0875e7a6e04010182a200581d61abde0f5259efacac08c88bd8c951eaad7b15d898a2a482f0ba3b7f16011a069db9c0a200581d6180f9e2c88e6c817008f3a812ed889b4a4da8e0bd103f86e7335422aa011a34fad460021a00023be00e81581c80f9e2c88e6c817008f3a812ed889b4a4da8e0bd103f86e7335422aa9fff8080f5f6"
}
```

### Signed transaction

[Link to file](txs/tx-bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb.signed)

```shell
{
    "type": "Tx BabbageEra",
    "description": "",
    "cborHex": "84a40081825820fb03abe73ddca76bc2f4a4fd18fde3b8e7844d7d1e3049042b4ed0875e7a6e04010182a200581d61abde0f5259efacac08c88bd8c951eaad7b15d898a2a482f0ba3b7f16011a069db9c0a200581d6180f9e2c88e6c817008f3a812ed889b4a4da8e0bd103f86e7335422aa011a34fad460021a00023be00e81581c80f9e2c88e6c817008f3a812ed889b4a4da8e0bd103f86e7335422aaa100818258205d010cf16fdeff40955633d6c565f3844a288a24967cf6b76acbeb271b4f13c15840e6766adf71231ec80faddbe12dcea623fd6bc31982cdbc69e90fb8c4dd937d4cdc87c2d3287a1c62be928a4ec01b970099410301adba27ca20fee0c08f68e50af5f6"
}
```

### Transaction views

#### Body

Result of calling

```shell
cardano-cli transaction view --tx-body-file tx-bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb.raw
```

```shell
auxiliary scripts: null
certificates: null
collateral inputs: []
era: Babbage
fee: 146400 Lovelace
inputs:
- fb03abe73ddca76bc2f4a4fd18fde3b8e7844d7d1e3049042b4ed0875e7a6e04#1
metadata: null
mint: null
outputs:
- address: addr1vx4aur6jt8h6etqgez9a3j23a2khk9wcnz32fqhshgah79swzdsp9
  address era: Shelley
  amount:
    lovelace: 111000000
  datum: null
  network: Mainnet
  payment credential key hash: abde0f5259efacac08c88bd8c951eaad7b15d898a2a482f0ba3b7f16
  reference script: null
  stake reference: null
- address: addr1vxq0nckg3ekgzuqg7w5p9mvgnd9ym28qh5grlph8xd2z92su77c6m
  address era: Shelley
  amount:
    lovelace: 888853600
  datum: null
  network: Mainnet
  payment credential key hash: 80f9e2c88e6c817008f3a812ed889b4a4da8e0bd103f86e7335422aa
  reference script: null
  stake reference: null
required signers (payment key hashes needed for scripts):
- 80f9e2c88e6c817008f3a812ed889b4a4da8e0bd103f86e7335422aa
update proposal: null
validity range:
  lower bound: null
  upper bound: null
withdrawals: null
```

#### Singed

Result of calling

```shell
cardano-cli transaction view --tx-file tx-bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb.signed
```

```shell
auxiliary scripts: null
certificates: null
collateral inputs: []
era: Babbage
fee: 146400 Lovelace
inputs:
- fb03abe73ddca76bc2f4a4fd18fde3b8e7844d7d1e3049042b4ed0875e7a6e04#1
metadata: null
mint: null
outputs:
- address: addr1vx4aur6jt8h6etqgez9a3j23a2khk9wcnz32fqhshgah79swzdsp9
  address era: Shelley
  amount:
    lovelace: 111000000
  datum: null
  network: Mainnet
  payment credential key hash: abde0f5259efacac08c88bd8c951eaad7b15d898a2a482f0ba3b7f16
  reference script: null
  stake reference: null
- address: addr1vxq0nckg3ekgzuqg7w5p9mvgnd9ym28qh5grlph8xd2z92su77c6m
  address era: Shelley
  amount:
    lovelace: 888853600
  datum: null
  network: Mainnet
  payment credential key hash: 80f9e2c88e6c817008f3a812ed889b4a4da8e0bd103f86e7335422aa
  reference script: null
  stake reference: null
required signers (payment key hashes needed for scripts):
- 80f9e2c88e6c817008f3a812ed889b4a4da8e0bd103f86e7335422aa
update proposal: null
validity range:
  lower bound: null
  upper bound: null
withdrawals: null
witnesses:
- key: VKey (VerKeyEd25519DSIGN "5d010cf16fdeff40955633d6c565f3844a288a24967cf6b76acbeb271b4f13c1")
  signature: SignedDSIGN (SigEd25519DSIGN "e6766adf71231ec80faddbe12dcea623fd6bc31982cdbc69e90fb8c4dd937d4cdc87c2d3287a1c62be928a4ec01b970099410301adba27ca20fee0c08f68e50a")
```

#### Witness

Witness built with `cardano-cli`.

[Link to file](txs/tx-witness-bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb.wit)

```shell
{
    "type": "TxWitness BabbageEra",
    "description": "",
    "cborHex": "82008258205d010cf16fdeff40955633d6c565f3844a288a24967cf6b76acbeb271b4f13c15840e6766adf71231ec80faddbe12dcea623fd6bc31982cdbc69e90fb8c4dd937d4cdc87c2d3287a1c62be928a4ec01b970099410301adba27ca20fee0c08f68e50a"
}
```
