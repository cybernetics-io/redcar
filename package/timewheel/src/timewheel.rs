use std::time::{Duration};

use std::collections::HashMap;
use crate::timer::{Timer};

const DEFAULT_SLOT_MAX: usize = 1024;
const DEFAULT_INTERVAL: u64 = 1;

pub struct TimeWheel<T>
    where T: Fn(u32) -> u32
{
    interval: Duration,
    slots: Vec<Timer>,
    timers: HashMap<u64, usize>,
    cursor: usize,
    slot_nums: usize,
    handler: T,
    done: bool
}

impl<T> TimeWheel<T>
    where T: Fn(u32) -> u32
{
    pub fn new(interval: u64, slot_num: usize, handler: T) -> TimeWheel<T> {
        let mut tw = TimeWheel{
            interval: Duration::from_secs(DEFAULT_INTERVAL),
            slots: Vec::new(),
            timers: HashMap::new(),
            cursor: 0,
            slot_nums: DEFAULT_SLOT_MAX,
            handler,
            done: false
        };
        if slot_num > 0 {
            tw.slot_nums = slot_num
        }
        if interval > 0 {
            tw.interval = Duration::from_secs(interval)
        }
        tw.init();
        tw
    }

    pub fn init(mut self) {
        for i in 0 .. self.slot_nums {
            self.slots.push(Timer::new());
        }
    }

    fn run(mut self) {
        let timer: Timer = self.slots.get(self.cursor).unwrap()?;
        timer.executor();
        if self.cursor == self.slot_nums - 1 {
            self.cursor = 0
        } else {
            self.cursor = self.cursor+1
        }
    }

    pub fn start() {

    }

    pub fn stop() {

    }
}
