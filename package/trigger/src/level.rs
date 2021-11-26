use std::collections::BinaryHeap;
use std::collections::LinkedList;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::Sender;
use proto::txn::Event;
use crate::{Action, Type};

pub struct Level {
    pub t: Type,
    pub sender: Arc<Mutex<Sender<Event>>>,
}

impl Action for Level {
    fn notify(&self, e: Event) {
        match self.sender.lock().unwrap().send(e) {
            Ok(_) => {}
            Err(err) => {
                println!("edge notice error")
            }
        }
    }
}
