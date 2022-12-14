# Milestone 1 interim report

During the research phase, several existing solutions were studied, such as Trezor, Ledger, BitBox02, and several smaller ones.
Summing up our findings we can conclude that from the high level perspective complete hardware wallet can consist of the following parts:
* Encrypted storage
* Usb interface
* Api
* Client transport library
* Client ui
* Etc

Although the scope of the project is not the full fledged hardware wallet, but the SDK that should provide developer tools for Cardano blockchain interactions on embedded devices, it is hard to draw exact border where SDK should “end”, i.e.
which of the parts mentioned above should be provided by SDK and which parts should be implemented by developers using it.

Ideally, we want to implement most of them in a general way to give developers flexible tools. We will start from implementing core functionality and then expand it gradually (e.g. adding USB interface, transport library etc..) as we go.

## Some technical details

There are no 100% ready Cardano solutions in Rust, especially ones that can work without standard library (no_std support), which is required by embedded systems. Solving this problem will be our first goal.

While implementing the USB interface requires a lot of routine work, it appears to be the main mode of interaction with embedded devices and most likely should be part of the SDK.
The way how it should be implemented is still uncertain and needs further study. Especially in the case of the WebUSB interface that is supported so far only by google chrome and could be deprecated in the future.


## Further steps

In order to achieve core SDK functionality providing few crates with no_std support would be a good start.
In particular, we will implement:
* Key management crate with hierarchical deterministic wallets (bip32 ed25519) support 
* Crate that will provide tools for Cardano transactions (and arbitrary data) signing

While we consider what other parts should and can be implemented, the SDK design highly likely will go through several iterations of refining.
