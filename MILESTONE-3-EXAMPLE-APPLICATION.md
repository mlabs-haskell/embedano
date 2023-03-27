# Example Application

## Idea

The device can measure temperature and sign measurements, and the host can post these measurements together with the timestamp and signature on-chain.

## Project Description

The project consists of two parts: a host application and device firmware. After the firmware is flashed to the device, the device can be connected to a PC with the host application via USB to perform necessary communication. The firmware provides sensor data upon request and uses Embedano SDK methods under the hood for keys derivation and signing. The firmware does not assemble transactions; it only provides sensor data together with required signatures and keys. Transaction assembly and submission happen on the host.

`Device` - embedded devices connected to Host via USB. Device is flashed with firmware which is built with Embedano SDK, enabling the device to derive keys and sign data.

`Host` - device (e.g., PC or laptop) that sends requests to embedded devices connected via USB using the host application.

`Script address` - the address of the script which is used as on-chain storage or registry for the data collected by `Host` from the `Device`.

`Device address` - [enterprise address](https://docs.cardano.org/learn/cardano-addresses) derived from the mnemonic phrase for the specified derivation path. This address is used to pay the minimum required Ada and balance transactions to the `Script address`. Mnemonics and derivation paths are set by the `Host` during requests to `Device`.

## Prerequisites

- `Device address` should have some Ada, so `Host` can balance transactions properly.
- `cardano-cli` of version `1.35.4` with access to appropriate node socket should be available in `PATH` on the `Host` - communications with cardano node (UTXO queries or submission) are performed with `cardano-cli` under the hood.

## Workflow

### Submitting Data to Chain

- `Host` initiates `Device` with the mnemonic phrase that will be used by Embedano SDK for keys derivation and signing.
- `Host` requests temperature sensor readings from `Device`, providing the timestamp of the operation (current time), and data required for keys derivation and signing (password and derivation path).
- `Device` returns temperature data, timestamp, and concatenated bytes of both signed with private key.
- `Host` requests a public key from `Device` with the same signing credentials as in the previous request to build `Device address`.
- `Host` queries `Device address` for available UTXOs and uses those and temperature data to build and balance the transaction - temperature data received from `Device` added to script output's Datum.
- `Host` calculates the transaction hash - transaction ID, and sends a request to `Device` to sign this ID.
- After receiving the signed transaction ID from `Device`, `Host` adds signed data to the witness set of balanced transaction, making the transaction signed.
- `Host` submits the signed transaction.

### Checking Data on Chain

- `Host` initiates `Device` with the mnemonic phrase that will be used by Embedano SDK for keys derivation and signing.
- `Host` queries `Script address` to get all available UTXOs (we assume that only UTXOs with temperature measurements are there).
- `Host` requests a public key from `Device` with the signing credentials that should correspond to the data stored on-chain (password and derivation path for key).
- For each UTXO from `Script address`, `Host` parses temperature data, timestamp, and `signed data` from UTXO Datum. Then performs bytes concatenation for temperature data and timestamp, making `verification data`. Using the public key acquired from `Device`, `verification data`, and `signed data`, `Host` checks that temperature data and timestamp were indeed signed by the public key acquired from `Device`.
