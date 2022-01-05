use std::ptr::null;
use std::time::Duration;

struct Task {
    delay: Duration,
    circle: i32,
    key: String,
    data: String
}

pub struct Timer<T>
    where T: Fn(u32) -> u32
{
    tasks: Vec<Task>,
    handler: T
}

impl<T> Timer<T>
    where T: Fn(u32) -> u32
{
    pub fn new(handler: T) -> Timer<T> {
        Timer {
            tasks: Vec::new(),
            handler
        }
    }

    pub fn add(self) {

    }

    pub fn del(self) {

    }

    pub fn executor(mut self) {
        for (i, task) in self.tasks.iter().enumerate() {
            if task.circle > 0 {
                *task.circle = task.circle - 1;
                continue
            }
            self.tasks.remove(i);
            if task.key != null() {

            }
        }
    }
}
