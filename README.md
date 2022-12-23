# Embedano

Secure and open-source software platform for embedded devices on Cardano blockchain, which can be used for hardware wallets and other devices buildable using its primitives.

[Catalyst Fund9 page](https://cardano.ideascale.com/c/idea/414017)

[Design doc](design-doc.md)


## Minimal keygen example with emulator
```bash
sudo apt-get update && sudo apt-get install qemu-system-arm
rustup toolchain install nightly
rustup +nightly target add thumbv7m-none-eabi
cargo +nightly run
```