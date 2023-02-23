# Example application

**Idea:** Enable posting and storing some sensor data on-chain.

**App structure:** A module that can provide sensor data upon request and use SDK methods under the hood. The app does not assemble transactions; it just provides sensor data together with required signatures and keys. Transaction assembly and submission happen on the host, which communicates with the embedded device.

`User` is a party that can communicate with the embedded `Device` from the host. `PK` is the payment key of `User`.

**Prerequisites:**

- The address of `PK` should have some ADA for the min-ADA constraint.

**Workflow:**

- `User` initiates `Device` with a seed of their funded wallet.
- `User` requests PK for a funded address from `Device` (or just use the corresponding derivation path in future calls).
- `User` requests sensor information from `Device`, providing their PK (or corresponding derivation path).
- `Device` responds with data that contains (`sensor_data`, `data_signature`). To make `data_signature`:
  - Hash `sensor_data` with a good enough hash function known to `User` that can be used on-chain (sha2_256, sha3_256, blake2b_256).
  - Sign hashed data with a private key that corresponds to `User`'s PK (can be found by SDK function / or if go with derivation path - derive key and sign).
- After getting the response, `User` can verify signature correctness with their `PK` and known hash function.
- `User` can then build a transaction to post `Device` response on-chain, placing (`sensor_data`, `data_signature`) in `Datum`. We can probably omit validation altogether as something "out of the scope," but in case we need it, something like:
  - Some `validator` address will serve as the keeper of sensor data records.
  - Probably use `minting policy` to verify UTXO **creation** correctness, mint single token per transaction; tokens can be used to find and collect data from `validator` address.
  - `Minting policy` can use `verifyEd25519Signature` to verify `data_signature` using `User`'s PK and hash functions available on-chain.
  - `Validator` can allow deletion of records by spending UTXO with sensor data record.
