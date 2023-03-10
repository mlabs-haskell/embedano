Make it part of query and verification of script datums if needed

```rust
    let socket = String::from("./cardano-node.socket");
    let node_client = node_client::CliNodeClient::new(socket, Network::Preprod);

    let s_addr = 
        Address::from_bech32("addr_test1wrt5qr8gwql8keruaulw69480tsx5er355gzpwk2u9k4v4czsac7e")
            .unwrap();

    let derivation_path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();

    let device = device_dummy::DeviceDummy::init(
        "initial label sand movie check train leaf escape hurt sort remove risk",
    );
    let pub_key = device.get_pub_key("", &derivation_path);

    let res = node_client.query_raw_inputs(&s_addr).unwrap();
    let res: HashMap<String, Value> = serde_json::from_str(&res).unwrap();
    println!("RESULT: {:?}\n", res);

    for v in res.into_values() {
        let datum = &v["inlineDatum"].to_string();
        let datum =  PlutusData::from_json(datum, PlutusDatumSchema::DetailedSchema).unwrap();
        println!("datum {:?}", datum);
        let datum = datum.as_list().unwrap();
        let num = datum.get(0).as_integer().unwrap();
        let num = from_str::<i8>(num.to_str().as_str()).unwrap();
        println!("num {:?}", num);
        
        let sig = datum.get(1).as_bytes().unwrap();
        let sig = ec::Ed25519Signature::from_bytes(sig).unwrap();
        println!("sig {:?}", sig);

        let ver = pub_key.verify(&num.to_ne_bytes(), &sig);
        println!("ver {:?}", ver);
    }
```