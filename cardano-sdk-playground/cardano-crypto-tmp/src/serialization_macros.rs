#[macro_export]
macro_rules! from_bytes {
    // Custom from_bytes() code
    ($name:ident, $data: ident, $body:block) => {
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        impl $name {
            pub fn from_bytes($data: Vec<u8>) -> Result<$name, DeserializeError> $body
        }
    };
    // Uses Deserialize trait to auto-generate one
    ($name:ident) => {
        from_bytes!($name, bytes, {
            let mut raw = Deserializer::from(std::io::Cursor::new(bytes));
            Self::deserialize(&mut raw)
        });
    };
}

#[macro_export]
macro_rules! to_bytes {
    ($name:ident) => {
        impl $name {
            pub fn to_bytes(&self) -> Vec<u8> {
                let mut buf = Serializer::new_vec();
                self.serialize(&mut buf).unwrap();
                buf.finalize()
            }
        }
    };
}

#[macro_export]
macro_rules! from_hex {
    // Custom from_bytes() code
    ($name:ident, $data: ident, $body:block) => {
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        impl $name {
            pub fn from_hex($data: &str) -> Result<$name, DeserializeError> $body
        }
    };
    // Uses Deserialize trait to auto-generate one
    ($name:ident) => {
        from_hex!($name, hex_str, {
            let mut raw = Deserializer::from(std::io::Cursor::new(hex::decode(hex_str).unwrap()));
            Self::deserialize(&mut raw)
        });
    };
}

#[macro_export]
macro_rules! to_hex {
    ($name:ident) => {
        impl $name {
            pub fn to_hex(&self) -> String {
                let mut buf = Serializer::new_vec();
                self.serialize(&mut buf).unwrap();
                hex::encode(buf.finalize())
            }
        }
    };
}

#[macro_export]
macro_rules! to_from_bytes {
    ($name:ident) => {
        to_bytes!($name);
        from_bytes!($name);
        to_hex!($name);
        from_hex!($name);
    };
}
