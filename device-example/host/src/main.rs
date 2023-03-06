use core::time::Duration;
use std::thread;

use cardano_embedded_sdk::api as embedano;
use cardano_embedded_sdk::bip::bip39::{dictionary, Entropy, Mnemonics};
use cardano_embedded_sdk::crypto::Ed25519Signature;
use cardano_embedded_sdk::types::TxId;
use derivation_path::DerivationPath;

use serialport::SerialPort;

use minicbor::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode)]
pub enum In {
    #[n(0)]
    Init(#[n(0)] String),
    #[n(1)]
    Sign(#[n(0)] Vec<u8>, #[n(1)] Vec<u8>, #[n(2)] String),
    #[n(2)]
    Verifiy(
        #[n(0)] Vec<u8>,
        #[n(1)] Vec<u8>,
        #[n(2)] Vec<u8>,
        #[n(3)] String,
    ),
}

#[derive(Clone, Debug, Encode, Decode)]
pub enum Out {
    #[n(0)]
    Init,
    #[n(1)]
    Sign(#[n(0)] Vec<u8>),
    #[n(2)]
    Verifiy(#[n(0)] bool),
    #[n(3)]
    Error(#[n(0)] String),
    #[n(4)]
    Length(#[n(0)] u64),
    #[n(5)]
    Read(#[n(0)] u64),
}

fn main() {
    let mnemonics = "aim wool into nose tell ball arm expand design push elevator multiply glove lonely minimum";
    let password = b"embedano";
    let entropy =
        Entropy::from_mnemonics(&Mnemonics::from_string(&dictionary::ENGLISH, mnemonics).unwrap())
            .unwrap();
    let path: DerivationPath = "m/1852'/1815'/0'/0/0".parse().unwrap();
    let (_prv_key, pub_key) = embedano::derive_key_pair(&entropy, password, &path);

    let tx_id =
        TxId::from_hex("bb1eb401cd03b0cd8caa08997df0a2ab226772c4d3a08adfb5a60ba34de12dfb").unwrap();

    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("/dev/tty.usbmodemTEST1", 115_200)
        .timeout(Duration::from_secs(10 * 60))
        .open()
        .expect("Failed to open port");

    println!("sending init");
    send(&mut port, In::Init(mnemonics.to_string()));
    println!("recieving init");
    recieve(&mut port);
    println!("recieved init");

    println!("sending sign");
    send(&mut port, In::Sign(
            tx_id.to_bytes().to_vec(),
        password.to_vec(),
        path.to_string(),
    ));
    println!("recieving sign");
    recieve(&mut port);
    println!("recieved sign");

//    if let Out::Sign(signature) = result {
//        let signature = Ed25519Signature::from_bytes(signature).expect("Decode signature failed!");
//        println!(
//            "Signature: {}, verified: {}",
//            signature.to_hex(),
//            pub_key.verify(tx_id.to_bytes(), &signature)
//        );
//
//        let data = minicbor::to_vec(In::Verifiy(
//            tx_id.to_bytes().to_vec(),
//            signature.to_bytes(),
//            password.to_vec(),
//            path.to_string(),
//        ))
//        .unwrap();
//        println!("In::Verifiy: {}", data.len());
//        let write = minicbor::to_vec(In::Write(data.len())).unwrap();
//        port.write_all(&write).expect("Write In::Write failed!");
//        port.write_all(&data).expect("Write In::Verifiy failed!");
//        port.read(buf.as_mut_slice())
//            .expect("Found no In::Verifiy data!");
//        let result: Out = minicbor::decode(&buf).unwrap();
//        println!("{result:#?}");
//    } else {
//        println!("Signing failed!")
//    }
}

fn send(port: &mut Box<dyn SerialPort>, value: In) {
    let data = minicbor::to_vec(&value).unwrap();
    let len = data.len();
    port.write(&(len as u64).to_be_bytes()).unwrap();
    port.write_all(&data).unwrap();
//    let mut offset = 0;
//    while offset < len {
//        while let Ok(count) = port.write(&data[offset..len]) {
//            if count == 0 {
//                break;
//            }
//            offset += count;
//            thread::sleep(Duration::from_millis(10));
//        }
//    }
    port.flush().unwrap();
    println!("{value:#?}\nSent: {len}");
}

fn recieve(port: &mut Box<dyn SerialPort>) {
    println!("recieve a");


    let mut length = [0u8; 8];
    if port.read_exact(&mut length).is_ok() {
        let length = u64::from_be_bytes(length);
        let mut buf = [0u8; 4096];
        let mut data = vec![];
        let mut read = 0;
        println!("length :{length:#?}");
        while let Ok(count) = port.read(&mut buf) {
            println!("recieve :{count:#?}");
            if count == 0 {
                break;
            }
            data.extend_from_slice(&buf[..count]);
            read += count;
            println!("read: {read}");
            if (read as u64) == length {
                break;
            }
            match minicbor::decode::<'_, Out>(&data[..read]) {
                Ok(v) => println!("Recieved {v:#?}"),
                e => println!("Recieved err: {e:#?}"),
            }
        }
        let result: Out = minicbor::decode(&data[..read]).unwrap();
        println!("Recieved {result:#?}");
    }
}

//fn send(port: &mut Box<dyn SerialPort>, value: In) {
//    let data = minicbor::to_vec(&value).unwrap();
//    let len = data.len() as u64;
//    port.write(&len.to_be_bytes()).unwrap();
////    port.flush().unwrap();
//    port.write(&data).expect(&format!("Write failed!\n{value:#?}"));
////    port.flush().unwrap();
//    println!("{value:#?}\nSent: {len}");
//}

//fn recieve(port: &mut Box<dyn SerialPort>) {
//    let mut buf: Vec<u8> = vec![0; 1024];
//    let mut message: Vec<u8> = vec![];
//
//    let mut buf: Vec<u8> = vec![0; 4096];
//    port.read(buf.as_mut_slice()).unwrap();
////    while let Ok(count) = port.read(buf.as_mut_slice()) {
////        message.extend_from_slice(&buf[..count]);
////        buf = vec![0; 1024];
////    }
//
//    if !message.is_empty() {
//        let result: Out = minicbor::decode(&message).unwrap();
//        println!("Recieved {result:#?}");
//    }
//}
