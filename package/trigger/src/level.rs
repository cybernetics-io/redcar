use std::collections::BinaryHeap;
use std::collections::LinkedList;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::Sender;
use proto::txn::Event;
use crate::{Action, Type};

pub struct Level {
    t: Type,
    sender: Sender<Event>,
}

impl Action for Level {
    fn notify(&self, e: Event) {
        todo!()
    }
}
