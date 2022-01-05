// Copyright 2021 Redcar Project Authors. Licensed under Apache-2.0.

use std::sync::{Arc, Mutex};
use index::index::{Index};
use proto::txn::{Event, KeyValue};
use trigger::Trigger;
use utils::Error;

#[derive(Clone)]
pub struct KV {
    pub index: Index,
    pub trigger: Arc<Mutex<Trigger>>,
}

impl KV {
    pub fn new(index: Index, trigger: Trigger) -> KV {
        KV {
            index,
            trigger: Arc::new(Mutex::new(trigger)),
        }
    }

    pub fn set(&mut self, k: Vec<u8>, v: Vec<u8>) {
        match self.index.set(k.clone(), v.clone()) {
            Ok(_) => {}
            Err(_) => {}
        };
        self.trigger.lock().unwrap().add(KeyValue {
            key: k,
            create_revision: 0,
            mod_revision: 0,
            version: 0,
            value: v,
        })
    }

    pub fn get(&mut self, k: &[u8]) -> Vec<u8> {
        self.index.get(Vec::from(k))
    }
}
