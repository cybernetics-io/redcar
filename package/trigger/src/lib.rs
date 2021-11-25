pub mod edge;
pub mod level;

use std::collections::{HashMap, HashSet, LinkedList};
use std::sync::{Arc, Mutex};
use proto::txn::{Event, KeyValue};
use crate::edge::Edge;
use crate::level::Level;

#[derive(Clone)]
pub enum Type {
    Edge,
    Level,
}

pub trait Action {
    fn notify(&self, e: Event);
}

#[derive(Clone)]
pub struct Trigger {
    pub map: HashMap<Vec<u8>, Arc<Mutex<Vec<Box<dyn Action + Send + Sync>>>>>,
}

impl Trigger {
    pub fn new() -> Self {
        Trigger {
            map: HashMap::new()
        }
    }

    pub fn add(&self, kv: KeyValue) {
        match self.map.get(kv.key.as_slice()) {
            None => {
                println!("none add event")
            }
            Some(node) => {
                for action in node.lock().unwrap().iter() {
                    action.notify(
                        Event {
                            r#type: 0,
                            kv: Option::from(kv.clone()),
                        }
                    )
                }
            }
        }
    }

    pub fn register(&mut self, key: Vec<u8>, a: Box<dyn Action + Send + Sync>) {
        match self.map.get(key.as_slice()) {
            None => {
                let mut node = Vec::new();
                node.push(a);
                self.map.insert(key, Arc::new(Mutex::new(node)));
            }
            Some(node) => {
                node.lock().unwrap().push(a)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
