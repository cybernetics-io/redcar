use crate::Backend;

pub struct RocksDB {}

impl RocksDB {
    pub fn new() -> RocksDB {
        RocksDB {}
    }
}

impl Backend for RocksDB {
    fn insert(&self, k: Vec<u8>, v: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
        todo!()
    }

    fn search(&self, key: Vec<u8>) -> Vec<u8> {
        todo!()
    }
}
