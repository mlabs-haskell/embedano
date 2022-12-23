# Embedano project

## Table of content

- [Embedano project](#embedano-project)
  - [Table of content](#table-of-content)
  - [Introduction](#introduction)
  - [Functionality](#functionality)
    - [Core](#core)
    - [Additional](#additional)
    - [Tools](#tools)
  - [CIP-21 compatibility](#cip-21-compatibility)
  - [Hardware](#hardware)
  - [Technical overview](#technical-overview)
    - [Some considerations](#some-considerations)
  - [Further steps](#further-steps)

## Introduction

This document introduces Embedano – open-source Cardano Embedded Rust SDK. This SDK will provide developer tools for Cardano blockchain interactions on embedded devices.

Embedano does not aim to be a full-fledged hardware wallet, but rather a tool, that developers can use to build their own:

- Hardware wallet
- IoT device with access to the Cardano blockchain
- Wearable with access to the Cardano blockchain
- Hardware Secure Module / key storage / signing embedded device

Or add the ability to interact with the Cardano blockchain to an existing project.

## Functionality

### Core

The core functionality of Rust SDK will include:

- Generation, storing, and resetting of the seed phrase
- Keys derivation
- Public Payment Key queries
- Public Staking Key queries
- Cardano Address queries
- Cardano Transactions signing
- Signing of arbitrary data
- Proof of ownership for Public Key or Address
- Seed phrase query
- Setting seed phrase (for recovery)

### Additional

- **USB interface:** interaction with embedded devices (and especially Hardware Wallets) mostly happens via USB. We aim to provide an interface that will enable communication between core functionality on embedded devices and developer via USB

### Tools

- **Scaffold repository:** repository that includes functionalities described above, which can be used as starting point for the embedded device firmware project with Embedano SDK. Will also include necessary documentation on how to build and flash the firmware.

## CIP-21 compatibility

Ideally, we want to avoid any restrictions and be able to sign any kind of Cardano transactions, and be as powerful as, say, `cardano-wallet`. But this requires further investigations and testing with hardware. Moreover, it is important to note that some Cardano DEXes require transactions to be CIP-21 compatible, so they can be signed by some popular hardware wallet models.

Potentially, we can provide 2 APIs - for CIP-21 compatible transaction signing, where a transaction is streamed to the device, and an API for signing the whole transaction for devices that can fit it into memory.

## Hardware

We aim to provide Rust SDK compatible with the ARM Cortex-M series of processors.
Initial development and QA will be performed on chips based on ARM® Cortex™-M:

- STM32F3
- nRF52840

## Technical overview

During the research phase, several existing solutions were studied, such as Trezor, Ledger, BitBox02, and several smaller ones.
Summing up our findings we can conclude that from the high-level perspective complete hardware wallet can consist of the following parts:

- Encrypted storage
- USB interface
- API
- Client transport library
- Client UI

and so on.

Because the scope of the project is not the full-fledged hardware wallet, but SDK that should provide developer tools for Cardano blockchain interactions on embedded devices, it is hard to draw the exact border where SDK should “end”, i.e. which of the parts mentioned above should be provided by SDK and which parts should be implemented by developers using it.

Ideally, we want to implement most of them in a general way to give developers flexible tools. We will start by implementing core functionality and then expand it gradually (e.g. adding a USB interface, transport library, etc..) as we go.

### Some considerations

There are no 100% ready Cardano solutions in Rust, especially ones that can work without the standard library (i.e. provides [no_std support](https://docs.rust-embedded.org/book/intro/no-std.html)), which is required by embedded systems. Solving this problem will be our first goal.

While implementing the USB interface requires a lot of routine work, it appears to be the main mode of interaction with embedded devices and most likely should be part of the SDK.

The way how it should be implemented is still uncertain and needs further study. Especially in the case of the WebUSB interface that is supported so far only by google chrome and could be deprecated in the future.

## Further steps

To achieve core SDK functionality providing a few crates with no_std support would be a good start.
In particular, we will implement:

- Key management crate with hierarchical deterministic wallets (bip32 ed25519) support
- Crate that will provide tools for Cardano transactions (and arbitrary data) signing

While we consider what other parts should and can be implemented, the SDK design highly likely will go through several iterations of refining.