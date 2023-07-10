use alloc::vec::Vec;
use blake2::{digest::consts::U32, Blake2b, Digest};
use cardano_embedded_sdk::tx_stream::TxEntry;

type Hasher = Blake2b<U32>;

pub struct MockHahser {
    hasher: Hasher,
}

impl MockHahser {
    pub fn new() -> Self {
        let hasher = Hasher::new();
        Self { hasher }
    }

    pub fn add_entry(&mut self, entry: &TxEntry) {
        let r = minicbor::to_vec(entry).unwrap();
        self.hasher.update(r);
    }

    pub fn finalize(&mut self) -> Vec<u8> {
        let res = self.hasher.finalize_reset();
        res.to_vec()
    }

    pub fn final_tx_id(&mut self) -> Vec<u8> {
        self.finalize()
    }

    pub fn reset(&mut self) {
        self.hasher.reset()
    }
}
