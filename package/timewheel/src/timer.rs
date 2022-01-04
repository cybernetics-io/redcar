use std::time::Duration;

struct Task {
    delay: Duration,
    circle: i32,
    key: String,
    data: String
}

pub struct Timer {
    tasks: Vec<Task>
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            tasks: Vec::new()
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
            self.tasks.remove(i)
        }
    }
}
