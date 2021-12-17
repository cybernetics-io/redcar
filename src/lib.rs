// Copyright 2021 Redcar Project Authors. Licensed under Apache-2.0.

pub mod config;
pub mod daemon;
pub mod kv;
pub mod os;
pub mod service;
pub mod txn;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
