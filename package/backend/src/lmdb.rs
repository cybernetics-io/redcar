use lmdb::{Database, Environment, EnvironmentBuilder, Transaction};
use std::path::Path;
use std::string::FromUtf8Error;
use std::sync::{Arc, Mutex, RwLock};

use crate::Backend;

pub struct LMDB {
    env: Environment,
    db: Database,
}

impl LMDB {
    pub fn new(path: String) -> LMDB {
        let env = lmdb::Environment::new()
            .set_map_size(1024 * 1024 * 1024)
            .open(Path::new(path.as_str()))
            .unwrap();
        let db = env.open_db(None).unwrap();
        LMDB { env: env, db: db }
    }
}

impl Backend for LMDB {
    fn insert(&self, k: Vec<u8>, v: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
        let mut txn = self.env.begin_rw_txn().unwrap();
        let key = match String::from_utf8(k.to_vec()) {
            Ok(ks) => ks,
            Err(err) => {
                panic!(err)
            }
        };
        let val = match String::from_utf8(v.to_vec()) {
            Ok(vs) => vs,
            Err(err) => {
                panic!(err)
            }
        };
        txn.put(
            self.db,
            &format!("{}", key),
            &format!("{}", val),
            lmdb::WriteFlags::empty(),
        )
        .unwrap();
        txn.commit().unwrap();
        (k, v)
    }

    fn search(&self, key: Vec<u8>) -> Vec<u8> {
        let txn = self.env.begin_rw_txn().unwrap();
        let k = match String::from_utf8(key.to_vec()) {
            Ok(ks) => ks,
            Err(err) => {
                panic!(err)
            }
        };
        let val = txn.get(self.db, &format!("{}", k)).unwrap();
        Vec::from(val)
    }
}
