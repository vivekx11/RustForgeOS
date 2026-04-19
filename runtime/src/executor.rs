//! Task executor

use alloc::collections::VecDeque;
use crate::task::Task;

pub struct Executor {
    task_queue: VecDeque<Task>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            task_queue: VecDeque::new(),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        self.task_queue.push_back(task);
    }

    pub fn run(&mut self) {
        // TODO: Implement executor run loop
    }
}
