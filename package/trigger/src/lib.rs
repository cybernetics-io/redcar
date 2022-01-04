pub mod edge;
pub mod level;

use std::collections::{HashMap, HashSet, LinkedList};
use std::sync::{Arc, Mutex};
use proto::txn::{Event, KeyValue};
use tokio::sync::{watch, broadcast};
use tokio::sync::broadcast::Sender;

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
    pub ss: HashMap<Vec<u8>, broadcast::Sender<Event>>,
    pub map: HashMap<Vec<u8>, Arc<Mutex<Vec<Box<dyn Action + Send + Sync>>>>>,
}

impl Trigger {
    pub fn new() -> Self {
        Trigger {
            ss: HashMap::new(),
            map: HashMap::new(),
        }
    }

    pub fn add(&self, kv: KeyValue) {
        match self.map.get(kv.key.as_slice()) {
            None => {
                println!("none add event")
            }
            Some(node) => {
                for action in node.lock().unwrap().iter() {
                    action.notify(Event {
                        action: 0,
                        kv: Option::from(kv.clone()),
                    })
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
            Some(node) => node.lock().unwrap().push(a),
        }
    }

    pub fn edge(&mut self, key: Vec<u8>) -> watch::Receiver<Event> {
        let (rx, tx) = watch::channel(Event {
            action: 0,
            kv: None,
        });
        let a: Box<dyn Action + Send + Sync> = Box::new(Edge {
            t: Type::Edge,
            sender: Arc::new(Mutex::new(rx)),
        });
        match self.map.get(key.as_slice()) {
            None => {
                let mut node = Vec::new();
                node.push(a);
                self.map.insert(key, Arc::new(Mutex::new(node)));
            }
            Some(node) => node.lock().unwrap().push(a),
        }
        tx
    }

    pub fn level(&mut self, key: Vec<u8>) -> broadcast::Receiver<Event> {
        match self.ss.get(key.as_slice()) {
            None => {
                let (rx, tx) = broadcast::channel(1024);
                let v = self.ss.insert(key.clone(), rx);
                let vv = match v {
                    None => {
                        panic!("aa")
                    }
                    Some(s) => s,
                };
                let a: Box<dyn Action + Send + Sync> = Box::new(Level {
                    t: Type::Level,
                    sender: Arc::new(Mutex::new(vv)),
                });
                match self.map.get(key.as_slice()) {
                    None => {
                        let mut node = Vec::new();
                        node.push(a);
                        self.map.insert(key.clone(), Arc::new(Mutex::new(node)));
                    }
                    Some(node) => node.lock().unwrap().push(a),
                }
                tx
            }
            Some(sender) => sender.subscribe(),
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
