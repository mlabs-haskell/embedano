# Embedano

This is an open-source software platform for embedded devices on the Cardano blockchain. It can be used for hardware wallets and other devices that can be built using its primitives.

[Catalyst Fund9 page](https://cardano.ideascale.com/c/idea/414017)

## Documentation

- [Design doc](docs/design-doc.md)
- [Core library](./docs/embedano-api-tour.md)
- [Example project that uses core library and `cardano-cli` to enable Cardano network interactions on nRF52 Series chip](./MILESTONE-3-EXAMPLE-APPLICATION.md). Check out live demo in the "links" section.
- [Modification of example project that streams transaction body for signing](./MILESTONE-4-TRANSACTION-STREAMING.md). Check out live demo in the "links" section.

## Acknowledgements

The Embedano core library  [cardano-embedded-sdk](./cardano-embedded-sdk/)  partially uses code from the open-source libraries [cardano-serialization-lib](https://github.com/Emurgo/cardano-serialization-lib) and [rust-ed25519-bip32](https://github.com/typed-io/rust-ed25519-bip32). Both libraries were instrumental in saving a lot of effort. Thank you!
