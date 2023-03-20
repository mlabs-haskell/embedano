use minicbor::{Decode, Encode};

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
    Temp(#[n(0)] Vec<u8>, #[n(1)] String),
    #[n(4)]
    PubKey(#[n(0)] Vec<u8>, #[n(1)] String)
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
    #[n(6)]
    Temp(#[n(0)] i32, #[n(1)] Vec<u8>),
    #[n(7)]
    PubKey(#[n(0)] String)
}
