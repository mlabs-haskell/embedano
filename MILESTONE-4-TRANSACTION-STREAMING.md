# Transaction streaming

- [Transaction streaming](#transaction-streaming)
  - [Example application](#example-application)
    - [Current limitations](#current-limitations)
  - [Firmware and client application code changes](#firmware-and-client-application-code-changes)
  - [Running the demo](#running-the-demo)
    - [Starting USB device](#starting-usb-device)
    - [Running host client application](#running-host-client-application)
  - [Gaps and improvements](#gaps-and-improvements)
  - [Links](#links)

## Example application

Live demo in [links](#links) section.

Example device firmware and client application that demonstrate transaction streaming are located in [examples/nrf52-stream](./examples/nrf52-stream/) directory. Both firmware and client are built on top of [nrf52-demo](./examples/nrf52-demo/) examples which were delivered and [demonstrated](https://drive.google.com/drive/folders/1P8kPAvXWtOB8tDGSoNAiuJpSlz0tRNEs) as a part of the Milestone 3 deliverables.

The main difference now is that instead of calculating the transaction ID on the client (host) side and sending it to the device for signing, the following procedure is carried out:

- The client application builds an unsigned transaction and streams the transaction body (partially, see "limitations" below) to the device. The parts of the transaction body are streamed one after another in sequence.
- The device asks the user to confirm each part of the transaction body it receives. The device "screen" is emulated through debug logging.
- Using each confirmed part of the transaction body, the device calculates a rolling hash. If the user rejects any single entry, the process will be cancelled.
- When the user confirms all parts of the transaction body and there is nothing left to stream, the client signals the end of the stream and sends the password and key derivation path to the device.
- When the device receives the end-of-the-stream message, it finalizes the rolling hash computation and obtains the transaction ID. The device then asks the user (again through a "simulated" screen) to confirm the final transaction ID. If the ID is confirmed, the device uses the password, derivation path, and stored entropy (seed phrase) to sign the transaction ID and send the signature back to the client.
- Depending on the message received from the device, the client outputs either the signature or an error to the terminal.

For more information on why transaction body streaming is required and what tools are currently available, please refer to the [CIP-21 document](https://cips.cardano.org/cips/cip21/).

### Current limitations

During development, we realized that the amount of work required to write serialization for each part of the transaction body was greater than the resources available to us. The current implementation only streams transaction inputs and fees, so the resulting transaction ID will serve as an example and source of data to sign. The real transaction ID should contain a hash of the entire body.

The `cardano-embedded-sdk` has been extended by the `tx_stream.rs` module, which describes types that enable streaming of transaction inputs and fees over the USB connection. The client-device messaging protocol has also been extended to transfer new types described in the `cardano-embedded-sdk`, so both sides can act accordingly. The `tx_stream.rs` and messaging protocol can be further extended to enable serialization and transmission of the remaining parts required for the transaction body. The processing pipelines on both the device firmware and client applications in the example are also implemented in an extensible way.

## Firmware and client application code changes

There are several key changes in the firmware and client application codebase compared to the base `nrf52-demo` example:

1. To speed up the demo, the client application no longer queries temperature from the device. Instead, it uses a constant mock value (see `device::Device::query_mock_sensor_data`).
2. The client application no longer uses `cardano-cli` to calculate the transaction ID. Instead, the transaction body is partially streamed to the device, which then calculates the transaction ID (see `device::Device::stream_tx`).
3. The client application no longer sends the transaction ID for signing to the device. If the streaming was successful, the device calculates and signs the transaction ID. (`device::Device::sign_transaction_id` is removed).
4. The firmware now initializes two buttons that are used to confirm or reject transaction body parts streamed to the device, and to confirm or reject final transaction ID signing.
5. The transaction is no longer submitted to the chain because, due to the partial streaming, the transaction ID and resulting signature will not match the full transaction body. Therefore, the assembly of a fully signed transaction is also omitted. However, to run the demo, access to the running node is still required to get input UTXOs from the wallet.

## Running the demo

The easiest way to run the demo is to use Nix, as the repository provides a ready-to-go Nix setup. The following instructions use Nix with flake.

From the root of the project, enter the Nix shell:

```shell
nix develop
```

To flash firmware onto the device, some prior setup is required. The specifics of the setup will depend on your hardware and software. To see an example for the `NRF52 Development Kit board` and `WSL2 Debian`, check out the [live demo for Milestone 3](https://drive.google.com/drive/folders/1P8kPAvXWtOB8tDGSoNAiuJpSlz0tRNEs).

Current setup uses [this cargo config](./examples/nrf52-stream/stream-device/.cargo/config.toml) and [this script.gdb](./examples/nrf52-stream/stream-device/script.gdb) to run `gdb` and flash firmware when `cargo run` is executed.

### Starting USB device

To flash firmware, from the root of the repo switch to the device directory

```shell
cd examples/nrf52-stream/stream-device
```

Make sure device is connected to the `gdb` server and `script.gdb` has correct IP set through `"target extended-remote ..."` command. Then run `cargo run`.

If everything goes well, `script.gdb` should load firmware, start it and create a breakpoint right before entering the main function of the firmware and halt the execution there.

Type `c` or `continue` into the terminal. If everything went smoothly, you should see something like this:

```shell
(gdb) c
Continuing.

Breakpoint 1, stream_device::__cortex_m_rt_main_trampoline () at examples/nrf52-stream/stream-device/src/main.rs:38
38      #[entry]
(gdb) 
```

Type `c` or `continue` again. The firmware should initialize everything that is needed and start the main loop. If everything went fine, a new USB device should appear in the system, and the client application should be able to communicate with it through the newly opened serial port.

You can adjust `script.gdb` to not create a breakpoint and to run the firmware right away without needing to type `continue`.

### Running host client application

In the second terminal `cd` to main stream example directory:

```shell
cd examples/nrf52-stream
```

This directory contains script [stream_demo.sh](./examples/nrf52-stream/stream_demo.sh) serves as a shortcut to run the client application. You will need to pass device serial port as an argument. E.g.:

```shell
./stream_demo.sh "/dev/ttyACM0"
```

The application client will attempt to initialize the device, build the transaction, and stream it to the device three times so that each of the following scenarios can be tried:

- All streamed entries and the final transaction ID are confirmed - the device will return a signature.
- Any streamed entry is discarded by the user - the device will return an error and the streaming will be cancelled.
- The final transaction ID calculated by the device is discarded by the user - the device will return an error.

## Gaps and possible future improvements

- Full support for transaction body streaming is the biggest gap currently. The current codebase is extendable, but the required work is time-consuming.
- The current hasher used for rolling hash calculation is very basic. A production-ready solution should track what entities were serialized and what is the current entity to ensure transaction body confirms [CIP-21](https://cips.cardano.org/cips/cip21/). If it does not, streaming should be canceled, the rolling hash reset, and an error sent to the client.
- In both [nrf52-demo milestone 3](./examples/nrf52-demo/) and [nrf52-stream milestone 4](./examples/nrf52-stream/) examples, there is duplication in types describing communication protocols. This duplication can be extracted to its own package or become part of the core [cardano-embedded-sdk](./cardano-embedded-sdk/) library.
- [nrf52-demo milestone 3](./examples/nrf52-demo/) and [nrf52-stream milestone 4](./examples/nrf52-stream/) - examples can be merged into a single one after streaming for the whole transaction body is implemented.
- Testing can be further expanded with automated tests running on the real hardware. See the [corresponding issue](https://github.com/mlabs-haskell/embedano/issues/31).

## Links

- [Live demo](https://drive.google.com/drive/folders/1T7fvXyYRIQkTNdeAHwGr6hjAfevlB6pb?usp=drive_link)
- [CIP-21](https://cips.cardano.org/cips/cip21/)
