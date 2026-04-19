//! Simple round-robin process scheduler

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use spin::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

#[derive(Debug)]
pub struct Process {
    pub pid: u64,
    pub state: ProcessState,
    // TODO: Add context (registers, stack pointer, etc.)
}

pub struct Scheduler {
    processes: Vec<Process>,
    ready_queue: VecDeque<u64>,
    current_pid: Option<u64>,
    next_pid: u64,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            processes: Vec::new(),
            ready_queue: VecDeque::new(),
            current_pid: None,
            next_pid: 1,
        }
    }

    pub fn spawn(&mut self) -> u64 {
        let pid = self.next_pid;
        self.next_pid += 1;

        let process = Process {
            pid,
            state: ProcessState::Ready,
        };

        self.processes.push(process);
        self.ready_queue.push_back(pid);

        pid
    }

    pub fn schedule(&mut self) -> Option<u64> {
        self.ready_queue.pop_front()
    }
}

static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

pub fn init() {
    // Scheduler is already initialized via static
}

pub fn spawn_process() -> u64 {
    SCHEDULER.lock().spawn()
}

pub fn schedule() -> Option<u64> {
    SCHEDULER.lock().schedule()
}
