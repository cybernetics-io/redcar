use std::fmt::{Debug, Display};
use std::vec::Vec;

//use index::index::Index;

struct Txn {}

impl Txn {
    pub fn new() -> Txn {
        Txn {}
    }

    pub fn read<I, B>(&self, _i: I, _b: B) -> Vec<u8>
    where
        I: Display,
        B: Debug,
    {
        Vec::new()
    }

    pub fn write<I, B>(&self, _i: I, _b: B, _kv: Vec<u8>) {}
}
