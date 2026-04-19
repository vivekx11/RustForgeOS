//! Synchronization primitives

use spin::{Mutex, RwLock};

pub type SpinMutex<T> = Mutex<T>;
pub type SpinRwLock<T> = RwLock<T>;
