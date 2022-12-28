# Embedano Example Project

The example project will be a GitHub repository that will contain the source code for CLI (command line interface)  tool and firmware for ARM® Cortex™-M chip that can be built with Embedano SDK. The repository will also provide build scripts and instructions on how to build and flash the firmware and ready-to-go packages for some popular development boards. The project will be based on the scaffold repository, which is also part of the SDK toolset. The project will showcase core libraries' functionality in combination with other tools delivered under Embedano development.

The CLI example will be the “basic” one - it will show how to work with the core of SDK directly. It could be a thin wrapper written in Rust, that will use core SDK libraries as dependencies. Using this CLI app users will be able to generate root key for Hierarchical Deterministic (HD) wallet from mnemonic, derive keys and address, check that some address belongs to HD wallet, and sign transactions and data. The CLI app source code will demonstrate how core libraries can be used in general, without requiring any hardware setup. It will demonstrate (or even teach) users how to use SDK API and some best practices and common patterns.

This basic example can serve as a quick start guide for developers who already have their firmware, hardware, and peripheral setup ready and want to enable interactions with Cardano. It also can be a good starting point for developers who want to build their understanding of how keys and addresses derivations and signing work in Сardano via a hands-on approach.

Firmware for ARM® Cortex™-M chip will demonstrate full-featured end-to-end communication between the users’ host device (e.g. PC) and some concrete hardware (e.g. nRF52840 development kit) which has the core SDK flashed to it. The full example will contain:

- Firmware package composed of core SDK libraries that can be built and flashed to the device. There will be step-by-step instructions on how to build and flash firmware
- Step-by-step instructions on how to enable USB communication with the device
- User-facing client (probably, CLI application) that can perform communication with an embedded device from the host via USB and fully utilize the functionality of core libraries flashed to the device
- Bash-script snippets with aforementioned client call examples and necessary explanations

Instructions on how to prepare the environment required for the full example will be provided as part of the scaffold repository. We will also provide specific examples and instructions for the two main chips that will be used in the development: STM32F3 and nRF52840.

Optionally, the example project can also include instructions on how to run the firmware mentioned above on emulated devices such as [QEMU](https://www.qemu.org/) for those who want to have embedded device experience but currently do not have access to physical hardware.