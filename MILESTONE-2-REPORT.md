# Milestone 2 report

## Documentation

- [Embedano SDK API tour](./docs/embedano-api-tour.md)
- Rust docs are also available for SDK API and can be generated via standard tools: `cargo doc -p cardano-embedded-sdk`

## Test cases

Tests for transaction id signing, data signing, and proving ownership are located in [api.rs module](./cardano-sdk-playground/cardano-embedded-sdk/src/api.rs).

We also tested our implementation against reference data. Reference data is generated with IOG's CLI tools like `cardano-address` and `cardano-cli` for mnemonic `all all all all all all all all all all all all`.

We use the following reference data:

- extended root private key
- pair of extended private and public keys derived from root private key for path `/1852'/1815'/0'/0/0`
- transaction id that was signed by derived private key (transaction was built and submitted on private testnet using `cardano-cli` from [cardano-node](https://github.com/input-output-hk/cardano-node))
- signature for transaction above (signature was made with `cardano-cli` as well)

In tests, we use Embedano SDK to generate the same set of data as we have as a reference. Then we check that generated data is identical to the reference data.

Tests for reference data:

- [slip14-keys-test](./cardano-sdk-playground/cardano-embedded-sdk/tests/slip14-keys-test.rs)
- [slip14-sign-test](./cardano-sdk-playground/cardano-embedded-sdk/tests/slip14-sign-test.rs)

Full details on the reference data can be found [here](./slip14-data/README.md).
