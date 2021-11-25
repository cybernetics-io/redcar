use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, RwLock};
use std::vec::Vec;

use backend::Backend;
use utils::Error;

#[derive(Clone)]
pub struct Index {
    pub map: BTreeMap<Vec<u8>, Vec<u8>>,
    pub db: Arc<RwLock<Box<dyn Backend + Send + Sync>>>,
}

impl Index {
    pub fn new(db: Box<dyn Backend + Send + Sync>) -> Index {
        Index {
            map: BTreeMap::new(),
            db: Arc::new(RwLock::new(db)),
        }
    }

    pub fn set(&mut self, k: Vec<u8>, v: Vec<u8>) -> Result<(), Error> {
        let kv = self.db.write().unwrap().insert(k, v);
        self.map.insert(kv.0, kv.1);
        Ok(())
    }

    pub fn get(&mut self, key: Vec<u8>) -> Vec<u8> {
        match self.map.get(&key) {
            Some(value) => value.to_vec(),
            None => self.db.read().unwrap().search(key),
        }
    }
}
