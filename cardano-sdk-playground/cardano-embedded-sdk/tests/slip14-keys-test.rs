use cardano_embedded_sdk::bip::bip39;
use cardano_embedded_sdk::sdkapi::derive_key_pair;
use cardano_embedded_sdk::sdktypes::XPrvKey;
use derivation_path::DerivationPath;
use std::fs;
use std::panic;
use std::path::Path;
use std::process::Command;

const DATA_DIR: &str = "./tests/slip14-keys-test-data";

// Check root and derived account keys for account 0 against reference data
// generated with `cardano-address` IOG CLI tool from same mnemonics
#[test]
fn test_slip14_keys() {
    const MNEMONICS: &str = "all all all all all all all all all all all all";
    run_test(|| {
        let data_path = Path::new(DATA_DIR);

        // sanity check to make sure cardano-address uses same mnemonic to generate keys data
        let mnemonics = read_file_trimmed(data_path.join("slip14.mnemonic"));
        assert_eq!(MNEMONICS, mnemonics);

        // test root key
        let root_key_reference_hex = read_file_trimmed(data_path.join("root_key_hex"));

        let mnemonics = bip39::Mnemonics::from_string(
            &bip39::dictionary::ENGLISH,
            "all all all all all all all all all all all all",
        )
        .unwrap();

        let entropy = bip39::Entropy::from_mnemonics(&mnemonics).unwrap();
        let root_key = XPrvKey::from_entropy(&entropy, b"");

        assert_eq!(root_key_reference_hex, root_key.to_hex());

        //test account 0 derived private key
        let prv_key_reference_hex = read_file_trimmed(data_path.join("addr_0_xprv_hex"));
        let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
        let (acc_0_prv_key, acc_0_pub_key) = derive_key_pair(&entropy, b"", &path);

        assert_eq!(prv_key_reference_hex, acc_0_prv_key.to_hex());

        //test account 0 public key
        let pub_key_reference_hex = read_file_trimmed(data_path.join("addr_0_xpub_hex"));
        assert_eq!(pub_key_reference_hex, acc_0_pub_key.to_hex());
    })
}

// this setup is borrowed from Eric Opines from here:
// https://medium.com/@ericdreichert/test-setup-and-teardown-in-rust-without-a-framework-ba32d97aa5ab
fn run_test<T>(test: T) -> ()
where
    T: FnOnce() -> () + panic::UnwindSafe,
{
    setup();

    let result = panic::catch_unwind(|| test());

    // teardown();

    assert!(result.is_ok())
}

// generate reference data using `cardano-address`
fn setup() {
    let mut sh = Command::new("sh");
    sh.current_dir(DATA_DIR);
    sh.arg("./gen_data.sh").output().unwrap();
}

fn read_file_trimmed<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path)
        .map(|s: String| String::from(s.trim())) // FIXME: is there better way to do this?
        .unwrap()
}
