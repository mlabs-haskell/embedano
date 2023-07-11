# Example Application

- [Example Application](#example-application)
  - [Application](#application)
    - [Idea](#idea)
    - [Project Description](#project-description)
    - [Prerequisites](#prerequisites)
    - [Workflow](#workflow)
      - [Submitting Data to Chain](#submitting-data-to-chain)
      - [Checking Data on Chain](#checking-data-on-chain)
  - [Running the demo](#running-the-demo)
    - [Starting USB device](#starting-usb-device)
    - [Running host client application](#running-host-client-application)
  - [Links](#links)
    - [Transactions and data from live demo on CardanoScan](#transactions-and-data-from-live-demo-on-cardanoscan)


## Application

### Idea

The device can measure temperature and sign measurements, and the host can post these measurements together with the timestamp and signature on-chain.

### Project Description

The project consists of two parts: a host application and device firmware. After the firmware is flashed to the device, the device can be connected to a PC with the host application via USB to perform necessary communication. The firmware provides sensor data upon request and uses Embedano SDK methods under the hood for keys derivation and signing. The firmware does not assemble transactions; it only provides sensor data together with required signatures and keys. Transaction assembly and submission happen on the host.

`Device` - embedded devices connected to Host via USB. Device is flashed with firmware which is built with Embedano SDK, enabling the device to derive keys and sign data.

`Host` - device (e.g., PC or laptop) that sends requests to embedded devices connected via USB using the host application.

`Script address` - the address of the script which is used as on-chain storage or registry for the data collected by `Host` from the `Device`.

`Device address` - [enterprise address](https://docs.cardano.org/learn/cardano-addresses) derived from the mnemonic phrase for the specified derivation path. This address is used to pay the minimum required Ada and balance transactions to the `Script address`. Mnemonics and derivation paths are set by the `Host` during requests to `Device`.

### Prerequisites

- `Device address` should have some Ada, so `Host` can balance transactions properly.
- `cardano-cli` of version `1.35.4` with access to appropriate node socket should be available in `PATH` on the `Host` - communications with cardano node (UTXO queries or submission) are performed with `cardano-cli` under the hood.

### Workflow

#### Submitting Data to Chain

- `Host` initiates `Device` with the mnemonic phrase that will be used by Embedano SDK for keys derivation and signing.
- `Host` requests temperature sensor readings from `Device`, providing the timestamp of the operation (current time), and data required for keys derivation and signing (password and derivation path).
- `Device` returns temperature data, timestamp, and concatenated bytes of both signed with private key.
- `Host` requests a public key from `Device` with the same signing credentials as in the previous request to build `Device address`.
- `Host` queries `Device address` for available UTXOs and uses those and temperature data to build and balance the transaction - temperature data received from `Device` added to script output's Datum.
- `Host` calculates the transaction hash - transaction ID, and sends a request to `Device` to sign this ID.
- After receiving the signed transaction ID from `Device`, `Host` adds signed data to the witness set of balanced transaction, making the transaction signed.
- `Host` submits the signed transaction.

#### Checking Data on Chain

- `Host` initiates `Device` with the mnemonic phrase that will be used by Embedano SDK for keys derivation and signing.
- `Host` queries `Script address` to get all available UTXOs (we assume that only UTXOs with temperature measurements are there).
- `Host` requests a public key from `Device` with the signing credentials that should correspond to the data stored on-chain (password and derivation path for key).
- For each UTXO from `Script address`, `Host` parses temperature data, timestamp, and `signed data` from UTXO Datum. Then performs bytes concatenation for temperature data and timestamp, making `verification data`. Using the public key acquired from `Device`, `verification data`, and `signed data`, `Host` checks that temperature data and timestamp were indeed signed by the public key acquired from `Device`.

## Running the demo

The easiest way to run the demo is to use Nix, as the repository provides a ready-to-go Nix setup. The following instructions use Nix with flake.

From the root of the project, enter the Nix shell:

```shell
nix develop
```

To flash firmware onto the device, some prior setup is required. The specifics of the setup will depend on your hardware and software. To see an example for the `NRF52 Development Kit board` and `WSL2 Debian`, check out the [live demo](https://drive.google.com/drive/folders/1P8kPAvXWtOB8tDGSoNAiuJpSlz0tRNEs).

Current setup uses [this cargo config](./examples/nrf52-demo/embedano-device/.cargo/config.toml) and [this script.gdb](./examples/nrf52-demo/embedano-device/script.gdb) to run `gdb` and flash firmware when `cargo run` is executed.

### Starting USB device

To flash firmware, from the root of the repo switch to the device directory

```shell
cd examples/nrf52-demo/embedano-device
```

Make sure device is connected to the `gdb` server and `script.gdb` has correct IP set through `"target extended-remote ..."` command. Then run `cargo run`.

If everything goes well, `script.gdb` should load firmware and start main loop. A new USB device should appear in the system, and the client application should be able to communicate with it through the newly opened serial port.

### Running host client application

In the second terminal `cd` to main stream example directory:

```shell
cd examples/nrf52-demo
```

This directory contains script [submission_demo.sh](./examples/nrf52-demo/submission_demo.sh) serves as a shortcut to run the client application. You will need to pass device serial and `mode of operation` as arguments.

To submit transaction containing sensor readings from the device:

```shell
./submissionsubmission_demo.sh "/dev/ttyACM0" submit
```

To verify sensor readings stored on chain:

```shell
./submission_demo.sh "/dev/ttyACM0" verify
```

The application then will start selected scenario according to selected `mode of operation`. To get more information about modes please check out rust docs for `demo-client` or [live demo videos](https://drive.google.com/drive/folders/1P8kPAvXWtOB8tDGSoNAiuJpSlz0tRNEs).

## Links

- Live demo - [link](https://drive.google.com/drive/folders/1P8kPAvXWtOB8tDGSoNAiuJpSlz0tRNEs)
- [Cortex-M quick start template](https://github.com/rust-embedded/cortex-m-quickstart)
- Board [nRF52840 DK](https://www.nordicsemi.com/Products/Development-hardware/nrf52840-dk) - used in demo
- [gdb-multiarch debugger](https://howtoinstall.co/en/gdb-multiarch)
- [J-Link software pack](https://www.segger.com/products/debug-probes/j-link/tools/j-link-software/)
- [usbpid](https://learn.microsoft.com/en-us/windows/wsl/connect-usb) instruction

### Transactions and data from live demo on CardanoScan

- [1st submitted transaction](https://preprod.cardanoscan.io/transaction/90516bf936e764cbc2cc16164d706b4c542cacec76b9fc45c679b191e0fdd414) - id `90516bf936e764cbc2cc16164d706b4c542cacec76b9fc45c679b191e0fdd414`
- [2nd submitted transaction](https://preprod.cardanoscan.io/transaction/377f5e7bb8a2f865748a9456d6ad4ae9a6585dc94ea8b35e8a64dffc1e23ceab) - id `377f5e7bb8a2f865748a9456d6ad4ae9a6585dc94ea8b35e8a64dffc1e23ceab`
- [demo script address - addr_test1wr5qpejpzx7szat38a58v246jk6hmexcvnfza5nsdvperqgvjcfxd](https://preprod.cardanoscan.io/address/addr_test1wr5qpejpzx7szat38a58v246jk6hmexcvnfza5nsdvperqgvjcfxd)
