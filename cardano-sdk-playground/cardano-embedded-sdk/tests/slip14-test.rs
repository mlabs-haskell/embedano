use cardano_embedded_sdk::bip::bip39;
use cardano_embedded_sdk::sdkwallet::XPrvKey;
use std::fs;
use std::panic;
use std::path::Path;
use std::process::Command;

#[test]
fn test_against_cardano_address_slip14() {
    const MNEMONICS: &str = "all all all all all all all all all all all all";
    run_test(|| {
        let data_path = Path::new("./tests/slip14-test-data");

        // sanity check: cardano-address uses same mnemonic to generate keys data
        let mnemonics = fs::read_to_string(data_path.join("slip14.mnemonic")).unwrap();
        assert_eq!(MNEMONICS, mnemonics);

        let root_key_reference_hex = 
            fs::read_to_string(data_path.join("root_key_hex"))
            .map(|s: String| String::from(s.trim())) // FIXME: is there better way to do this?
            .unwrap();

        let root_key_reference_hex = root_key_reference_hex;

        let mnemonics = bip39::Mnemonics::from_string(
            &bip39::dictionary::ENGLISH,
            "all all all all all all all all all all all all",
        )
        .unwrap();

        let entropy = bip39::Entropy::from_mnemonics(&mnemonics).unwrap();
        let root_key = XPrvKey::from_entropy(entropy.as_ref(), b"");

        assert_eq!(root_key_reference_hex, root_key.to_hex())
    })
}
fn run_test<T>(test: T) -> ()
where
    T: FnOnce() -> () + panic::UnwindSafe,
{
    setup();

    let result = panic::catch_unwind(|| test());

    // teardown();

    assert!(result.is_ok())
}

fn setup() {
    let mut sh = Command::new("sh");
    sh.current_dir("./tests/slip14-test-data");

    let r = sh.arg("./gen_data.sh").output().unwrap();

    // let r = Command::new("ls")
    //     .output()
    //     .unwrap();

    println!("LS: {:?}", r);
}
