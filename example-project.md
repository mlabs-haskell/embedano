# Embedano Example Project

Example project will be in the form of a repository based on scaffold repository, which is part of the SDK tools. Project will showcase core libraries functionality in combination with other tools delivered under Embedano development.

Functionality can be demonstrated from several perspectives.

The “basic” example will show how to work with the core of SDK directly via some thin wrapper. It could be a CLI (command line interface) application written in Rust, that will use core SDK libraries as dependencies. Using this CLI app users will be able to exercise keys and address derivation from mnemonic, check that some address belongs to keys generated from known mnemonic, sign transactions and data. The CLI app source code will demonstrate how core libraries can be used in general, without requiring any hardware setup. It will demonstrate (or even teach) users how to use SDK API and some best practices and common patterns.

This basic example can serve as a quick start guide for developers who already have their firmware, hardware and peripheral setup ready and want to enable interactions with Cardano. It also can be a good starting point for developers who want to build their understanding on how keys and addresses derivations and signing works in Сardano via hands on approach.

The “full” example will demonstrate full featured end-to-end communication between users’ host device (e.g. PC) and some concrete hardware (e.g. nRF52840 development kit) which has the core SDK flashed to it. Full example will contain:

- Firmware package composed from core SDK libraries that can be built and flashed to the device. There will be step-by-step instructions on how to build and flash firmware
- Step-by-step instructions on how to enable USB communication with device
- User facing client (probably, in form of CLI) that can perform communication with embedded device from host via USB and fully utilize core functionality flashed to the device
- Bash-script snippets with aforementioned client calls examples with explanations

Instructions on how to prepare the environment required for the full example will be provided as part of the scaffold repository. We also include some concrete examples and instructions for two main chips that  we are going to use during development:: STM32F3 and nRF52840.

Optionally, the example project can also include instructions on how to run firmware mentioned above on emulated devices such as [QEMU](https://www.qemu.org/) for those who want to have embedded device experience but currently don’t have access to physical hardware.
