use std::time::{Duration};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{ Sender, Receiver };

use crate::timer::{Timer};

const DEFAULT_SLOT_MAX: usize = 1024;
const DEFAULT_INTERVAL: u64 = 1;

pub struct TimeWheel<T>
    where T: Fn(u32) -> u32
{
    interval: Duration,
    slots: Vec<Timer<T>>,
    timers: HashMap<String, usize>,
    cursor: usize,
    slot_nums: usize,
    handler: T,
    push_ticker: Sender<String>,
    push_timer: Sender<String>,
    poll_ticker: Receiver<String>,
    poll_timer: Receiver<String>,
    done: bool
}

impl<T> TimeWheel<T>
    where T: Fn(u32) -> u32
{
    pub fn new(interval: u64, slot_num: usize, handler: T) -> TimeWheel<T> {
        let (tx, rx) = mpsc::channel(32);
        let (wtx, wrx) = mpsc::channel(32);
        let mut tw = TimeWheel{
            interval: Duration::from_secs(DEFAULT_INTERVAL),
            slots: Vec::with_capacity(DEFAULT_SLOT_MAX),
            timers: HashMap::new(),
            cursor: 0,
            slot_nums: DEFAULT_SLOT_MAX,
            handler,
            push_ticker: tx,
            push_timer: wtx,
            poll_ticker: rx,
            poll_timer: wrx,
            done: false
        };
        if slot_num > 0 {
            tw.slot_nums = slot_num;
            tw.slots = Vec::with_capacity(slot_num)
        }
        if interval > 0 {
            tw.interval = Duration::from_secs(interval)
        }
        tw.init(||handler);
        tw
    }

    pub fn init(mut self, handler: T) {
        for i in 0 .. self.slot_nums {
            self.slots.push(Timer::new(||handler));
        }
    }

    pub fn add_timer(self) {
        self.push_timer.send(String::from("abc"))
    }

    fn run(mut self) {
        let timer: Timer<T> = self.slots.get(self.cursor).unwrap()?;
        timer.executor();
        if self.cursor == self.slot_nums - 1 {
            self.cursor = 0
        } else {
            self.cursor = self.cursor+1
        }
    }

    pub fn build(self) {

    }

    pub fn start(self) {
        tokio::spawn(async{
            tokio::select! {
                val = self.poll_ticker => {
                    self.run();
                    println!("ticker completed first with {:?}", val)
                }
                val = self.poll_timer => {
                    self.build();
                    println!("ticker completed first with {:?}", val);
                }
            }
        });
    }

    pub fn stop() {

    }
}
