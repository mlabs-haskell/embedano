use std::{thread, time::Duration};

use cardano_embedded_sdk::types::{TxId, XPubKey};
use derivation_path::DerivationPath;
use serialport::SerialPort;

use crate::serialization::{In, Out};
use std::fmt::Debug;

#[derive(Debug)]
pub struct DeviceData {
    pub sensor_readings: i32,
    pub signed_readings: Vec<u8>,
}

pub struct Device {
    port: Box<dyn SerialPort>,
}

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
    pub fn query_sensor_data(
        &mut self,
        password: &String,
        derivation_path: &DerivationPath,
    ) -> DeviceData {
        // println!("sending temp");
        let temp_request = In::Temp(password.as_bytes().to_vec(), derivation_path.to_string());
        send(&mut self.port, temp_request);
        let temp_data = receive(&mut self.port);
        match temp_data {
            Ok(Some(Out::Temp(sensor_readings, signed_readings))) => {
                println!("Device: getting sensor data - OK");

                DeviceData {
                    sensor_readings,
                    signed_readings,
                }
            }
            x => panic_to_unknown("Failed to get temperature data.", x),
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
        match XPubKey::from_hex(&pub_key_hex) {
            Ok(pk) => {
                println!("Device: getting public key - OK");
                pk
            }
            Err(err) => panic!("Device: Could not parse pub key from hex: {:?}", err),
        }
    }

    pub fn sign_transaction_id(
        &mut self,
        tx_id: &TxId,
        password: &String,
        derivation_path: &DerivationPath,
    ) -> Vec<u8> {
        let tx_id_sign_request = In::Sign(
            tx_id.to_bytes().to_vec(),
            password.as_bytes().to_vec(),
            derivation_path.to_string(),
        );
        send(&mut self.port, tx_id_sign_request);
        let result = receive(&mut self.port);
        match result {
            Ok(Some(Out::Sign(signature))) => {
                println!("Device: signing transaction ID - OK");
                signature
            }
            x => panic_to_unknown("Could not get Tx signature.", x),
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
    //    port.flush().unwrap();
    // println!("{value:#?}\nSent: {len}");
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
