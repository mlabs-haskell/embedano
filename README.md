# Embedano

Secure and open-source software platform for embedded devices on Cardano blockchain, which can be used for hardware wallets and other devices buildable using its primitives.

[Catalyst Fund9 page](https://cardano.ideascale.com/c/idea/414017)

[Design doc](docs/design-doc.md)

## Development

Current setup is based on [cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart) and requires Rust `nightly`.

### With Nix

Nix setup uses `flakes` feature.

To enable `flakes` add the following to Nix config:

```shell
experimental-features = nix-command flakes
```

Nix development shell will have nightly Rust and the following build targets available: `Cortex-M3`, `Cortex-M4/M7`, and `Cortex-M4F/M7F`.

To start development shell:

```shell
nix develop
```

This setup also includes a development shell with [QEMU emulator](https://www.qemu.org/) available. To run shell with QEMU:

```shell
nix develop .#withQemu
```

Be aware, that shell with QEMU will download QEMU (~1.5 Gb). Runnable example can be found in [qemu-example](examples/qemu-example/build.rs), e.g. from repository root run:

```shell
nix develop .#withQemu

# when shell is ready
cd examples/qemu-example/
cargo run --release
```

If everything works you should see `"Test: Generating keys"` and pair of keys in console output.

### Without Nix

If you want to install everything yourself from scratch please refer to [cortex-m-quickstart instructions](https://github.com/rust-embedded/cortex-m-quickstart).
