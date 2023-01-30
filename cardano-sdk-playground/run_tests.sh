#!/usr/bin/env bash


set -euo pipefail

cargo test -p cardano-embedded-sdk --test slip14-sign-test
cargo test -p cardano-embedded-sdk test_pair_check
cargo test -p cardano-embedded-sdk test_ownership
cargo test -p cardano-embedded-sdk test_ownership_wrong_seed
cargo test -p cardano-embedded-sdk test_ownership_out_of_account_gap
cargo test -p cardano-embedded-sdk test_ownership_out_of_address_gap
