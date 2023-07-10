# Transaction streaming

- [Transaction streaming](#transaction-streaming)
  - [Example application](#example-application)
    - [Current limitations](#current-limitations)
  - [Firmware and client application code changes](#firmware-and-client-application-code-changes)
  - [Running the demo](#running-the-demo)
    - [Starting USB device](#starting-usb-device)
    - [Running host client appliation](#running-host-client-appliation)
  - [Gap analysis](#gap-analysis)

## Example application

Example device firmware and client application that demonstrate transaction streaming are located in [examples/nrf52-stream](./examples/nrf52-stream/) directory. Both firmware and client are built on top of [nrf52-demo](./examples/nrf52-demo/) examples which were delivered and [demonstrated](https://drive.google.com/drive/folders/1P8kPAvXWtOB8tDGSoNAiuJpSlz0tRNEs) as a part of Milestone 3 deliverables.

The main difference is that now instead of calculating transaction ID on the client (host) side and then sending transaction ID to the device which signs it, the following happens:

- Client application will build unsigned transaction and stream transaction body (partially, see "limitations" below) to the device - parts of transaction body will be streamed one after another in sequence
- Device will ask user to confirm each part of the transaction body it receives - device "screen" is emulated through debug logging
- Using each confirmed part of the transaction body device will calculate rolling hash, if user rejects any single entry, the process will be cancelled
- When user confirms all parts of the transaction body and there is nothing left to stream, client will signal the end of the stream and send password and key derivation path to the device
- When device receives end-fo-the-stream message, it will finalize rolling hash computation and obtain transaction ID. Then device will ask the user (again through "simulated" screen) to confirm final transaction ID. If ID is confirmed, device will use password, derivation path and stored in memory entropy (seed phrase) to sign transaction ID and send signature back to the client.
- Depending on message received from the device, client will output signature or error to the terminal

### Current limitations

Due to the big amount of work required to write serialization for each and every part of the transaction body and lack of the resources, current example implements streaming only for transaction inputs and fee. So resulting transaction ID will serve just as an example and source of the data to sign (real transaction ID should contain hash of the whole body).

`cardano-embedded-sdk` was extended by `tx_stream.rs` module. It describes types that enable streaming of transaction inputs and fee over the USB connection. Client-device messaging protocol was also extended to be able to transfer new types described in `cardano-embedded-sdk` so both sides can act accordingly. `tx_stream.rs` and messaging protocol can be further extended to enable serialization and transmission of the rest required parts of transaction body.

## Firmware and client application code changes

There are couple key changes in the codebase of firmware and client application compare to the base `nrf52-demo` example:

1. To speed up the demo client application no longer queries temperature from the device, but instead returns constant mock value (see `device::Device::query_mock_sensor_data`)
2. Client application do not use `cardano-cli` anymore to calculate transaction ID, instead transaction body (partially) streamed to the device and device calculates transaction ID (see `device::Device::stream_tx`)
3. Client application no longer sends transaction ID for signing to the device, instead, if streaming went successfully, device calculates and signs transaction ID (`device::Device::sign_transaction_id` removed)
4. Firmware now initializes two buttons which are used to confirm or reject transaction body parts streamed to the device, and also confirm or reject final transaction ID signing
5. Transaction is no longer submitted to the chain, as due to the partial streaming transaction ID and resulting signature will not match full transaction body. Thus, the assembly of a fully signed transaction is also omitted. To run the demo, though, access to the running node is still required in order to get input UTXOs from the wallet.

## Running the demo

The easiest way to run the demo is to use Nix as it provides ready-to-go setup. The following instruction uses Nix with flake.

From the root of the project enter Nix shell

```shell
nix develop
```

To flash firmware some prior setup is required that will depend on yor hardware and software. To see example for `NRF52 development kit board` + `WSL2 Debian` check out [live demo for Milestone 3](https://drive.google.com/drive/folders/1P8kPAvXWtOB8tDGSoNAiuJpSlz0tRNEs).

Current setup uses [this cargo config](./examples/nrf52-stream/stream-device/.cargo/config.toml) and [this script.gdb](./examples/nrf52-stream/stream-device/script.gdb) to run `gdb` and flash firmware when `cargo run` is executed.

### Starting USB device

To flash firmware, from the root of the repo switch to the device directory

```shell
cd examples/nrf52-stream/stream-device
```

Make sure device is connected to the `gdb` server and `script.gdb` has correct IP set through `"target extended-remote ..."` command. Then run `cargo run`.

If everything goes fine, `script.gdb` should create breakpoint right before entering `main` function of the firmware and halt execution there. 

Type `c` or `continue` to the terminal, and if everything went fine you should see something like this:

```
(gdb) c
Continuing.

Breakpoint 1, stream_device::__cortex_m_rt_main_trampoline () at examples/nrf52-stream/stream-device/src/main.rs:38
38      #[entry]
(gdb) 
```

Type `c` or `continue` again. Now Firmware should initialize everything needed and start main loop. If everything went OK new USB device should appear in the system and client application should be able to communicate with it through newly opened serial port.

You can adjust `script.gdb` to not to create breakpoint and just run firmware right away without need to type `continue`.

### Running host client appliation

In the second terminal cd to main stream example directory:

```shell
cd examples/nrf52-stream
```

This directory contains script [submission_demo.sh](./examples/nrf52-stream/submission_demo.sh)` which is shortcut to run client application. You will need to pass device serial port as an argument. E.g.:

```shell
./submission_demo.sh "/dev/ttyACM0"
```

Application client then will attempt to initialize device, build transaction and stream it twice to the device so it is possible to try both scenarios:

- when all streamed entries and final transaction ID are confirmed - device will return signature
- when any streamed entry or transaction is is discarded - device will return an error and streaming will be cancelled

## Gap analysis

scratch:

- Full body streaming possibility
- Hasher should control what entries it hashes to make sure things are CIP-21 compliant
- Protocol schema duplication in device and client libraries - should be extracted to some "common" package?

