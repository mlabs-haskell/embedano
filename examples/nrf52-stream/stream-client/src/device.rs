use std::result;
use std::{thread, time::Duration};

use cardano_embedded_sdk::tx_stream::{TxEntry, TxStream};
use cardano_embedded_sdk::types::{TxId, XPubKey};
use cardano_serialization_lib::utils::BigNum;
use cardano_serialization_lib::{Transaction, TransactionInputs};
use derivation_path::DerivationPath;
use serialport::SerialPort;

use minicbor::{Decode, Encode};
use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH};

/// Incoming messages that device receives from host.
/// Serialized to CBOR.
/// Mirrors corresponding type in `lib.rs` in device package.
#[derive(Clone, Debug, Encode, Decode)]
pub enum In {
    #[n(0)]
    Init(#[n(0)] String),
    #[n(1)]
    Sign(#[n(0)] Vec<u8>, #[n(1)] Vec<u8>, #[n(2)] String),
    #[n(2)]
    Verify(
        #[n(0)] Vec<u8>,
        #[n(1)] Vec<u8>,
        #[n(2)] Vec<u8>,
        #[n(3)] String,
    ),
    #[n(3)]
    Temp(#[n(0)] Vec<u8>, #[n(1)] u64, #[n(2)] String),
    #[n(4)]
    PubKey(#[n(0)] Vec<u8>, #[n(1)] String),
    #[n(5)]
    Stream(#[n(0)] TxStream),
}

/// Outgoing messages that device sends to host.
/// Serialized to CBOR.
/// Mirrors corresponding type in `lib.rs` in device package.
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
    #[n(6)]
    Temp(#[n(0)] i32, #[n(1)] Vec<u8>),
    #[n(7)]
    PubKey(#[n(0)] String),
    #[n(8)]
    StreamResponse(#[n(0)] String),
}

#[derive(Debug)]
pub struct DeviceData {
    pub sensor_readings: i32,
    pub signed_readings: Vec<u8>,
    pub measure_time: u64,
}

pub struct Device {
    port: Box<dyn SerialPort>,
}

/// Wrapper around serial port for device with helper functions
impl Device {
    pub fn new(addr: &str) -> Self {
        let port = serialport::new(addr, 115_200)
            .timeout(Duration::from_millis(100000))
            .open()
            .expect("Failed to open port");

        Device { port: port }
    }

    pub fn init(&mut self, mnemonics: String) {
        send(&mut self.port, In::Init(mnemonics));
        let init = receive(&mut self.port);
        let _ = match init {
            Ok(Some(Out::Init)) => println!("Device: initialization - OK"),
            x => panic_to_unknown("Initialization failed!", x),
        };
    }

    /// Receive from device current temperature (°C) and its signed bytes
    pub fn query_mock_sensor_data(
        &mut self,
        _password: &String,
        _derivation_path: &DerivationPath,
    ) -> DeviceData {
        let measure_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        DeviceData {
            sensor_readings: 24,
            signed_readings: "ff".as_bytes().to_vec(),
            measure_time: measure_time,
        }
    }

    pub fn get_public_key(
        &mut self,
        password: &String,
        derivation_path: &DerivationPath,
    ) -> XPubKey {
        let pub_key_request = In::PubKey(password.as_bytes().to_vec(), derivation_path.to_string());
        send(&mut self.port, pub_key_request);
        let result = receive(&mut self.port);
        let pub_key_hex = match result {
            Ok(Some(Out::PubKey(key_hex))) => key_hex,
            x => panic_to_unknown("Could not get hex of public key", x),
        };
        // let pub_key_hex = "ced76eabfcda61c6e1e1b7fb5647df737bfac07ec10c9261f347a30ad618401156dc280f19712b6f0a2faff84285912ec539e2fdc8a90428bf8734f09a05fae4";
        match XPubKey::from_hex(&pub_key_hex) {
            Ok(pk) => {
                println!("Device: getting public key - OK");
                pk
            }
            Err(err) => panic!("Device: Could not parse pub key from hex: {:?}", err),
        }
    }

    pub fn stream_tx(
        &mut self,
        tx: &Transaction,
        password: &String,
        derivation_path: &DerivationPath,
    ) -> Result<Vec<u8>, String> {
        self.stream_inputs(tx.body().inputs())?;
        self.stream_fee(tx.body().fee())?;
        self.finalize_stream(password, derivation_path)
    }

    fn stream_inputs(&mut self, ins: TransactionInputs) -> Result<String, String> {
        for n in 0..ins.len() {
            let inp = ins.get(n);
            let hash = inp.transaction_id().to_bytes();
            let ix = inp.index();

            let inp_stream_req = In::Stream(TxStream::Entry(TxEntry::TxInput(hash, ix)));
            send(&mut self.port, inp_stream_req);

            match receive(&mut self.port) {
                Ok(Some(Out::StreamResponse(msg))) => {
                    println!("Device: streaming TxIn: {}", msg);
                }
                other => return Err(format!("Error streaming inputs: {:?}", other)),
            }
        }
        Ok("".into())
    }

    fn stream_fee(&mut self, fee: BigNum) -> Result<String, String> {
        let fee_request = In::Stream(TxStream::Entry(TxEntry::Fee(
            cardano_serialization_lib::utils::from_bignum(&fee),
        )));
        send(&mut self.port, fee_request);
        match receive(&mut self.port) {
            Ok(Some(Out::StreamResponse(msg))) => {
                println!("Device: streaming fee: {}", msg);
                Ok("".into())
            }
            other => Err(format!("Error streaming fee: {:?}", other)),
        }
    }

    fn finalize_stream(
        &mut self,
        password: &String,
        derivation_path: &DerivationPath,
    ) -> Result<Vec<u8>, String> {
        println!("Device: finalizing stream and asking to sign transaction ID");
        let done_request =
            TxStream::Done(password.as_bytes().to_vec(), derivation_path.to_string());
        send(&mut self.port, In::Stream(done_request));
        let result = receive(&mut self.port);

        match result {
            Ok(Some(Out::Sign(signature))) => {
                println!("Device: signing transaction ID - OK");
                Ok(signature)
            }
            err => Err(format!("Error while finalizing stream: {:?}", err)),
        }
    }
}

pub fn send(port: &mut Box<dyn SerialPort>, value: In) {
    let data = minicbor::to_vec(&value).unwrap();
    let len = data.len();
    port.write(&(len as u64).to_be_bytes()).unwrap();
    thread::sleep(Duration::from_millis(10));
    for chunk in data.chunks(64) {
        port.write(&chunk).unwrap();
    }
}

pub fn receive(port: &mut Box<dyn SerialPort>) -> Result<Option<Out>, String> {
    let mut length = [0u8; 8];
    if port.read_exact(&mut length).is_ok() {
        let length = u64::from_be_bytes(length);
        let mut buf = [0u8; 4096];
        let mut data = vec![];
        let mut read = 0;
        while let Ok(count) = port.read(&mut buf) {
            if count == 0 {
                break;
            }
            data.extend_from_slice(&buf[..count]);
            read += count;
            if (read as u64) == length {
                break;
            }
        }
        match minicbor::decode::<'_, Out>(&data[..read]) {
            Ok(v) => return Ok(Some(v)),
            e => return Err(format!("minicbor decode error: {e:#?}")),
        }
    }
    Ok(None)
}

fn panic_to_unknown<R, T>(msg: &str, x: T) -> R
where
    T: Debug,
{
    panic!("Device: {}. Device returned: {:?}", msg, x)
}
