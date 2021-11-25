use std::sync::{Arc, Mutex};
use tokio::sync::watch::Sender;
use proto::txn::Event;
use crate::{Action, Type};

pub struct Edge {
    pub t: Type,
    pub sender: Arc<Mutex<Sender<Event>>>,
}

impl Action for Edge {
    fn notify(&self, e: Event) {
        match self.sender.lock().unwrap().send(e) {
            Ok(_) => {}
            Err(err) => {
                println!("edge notice error")
            }
        }
    }
}
