use std::{thread, time::Duration};

use serialport::SerialPort;

use crate::serialization::{In, Out};

pub fn send(port: &mut Box<dyn SerialPort>, value: In) {
  let data = minicbor::to_vec(&value).unwrap();
  let len = data.len();
  port.write(&(len as u64).to_be_bytes()).unwrap();
  thread::sleep(Duration::from_millis(10));
  for chunk in data.chunks(64) {
      port.write(&chunk).unwrap();
  }
//    port.flush().unwrap();
  println!("{value:#?}\nSent: {len}");
}

pub fn recieve(port: &mut Box<dyn SerialPort>) -> Result<Option<Out>, String> {
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