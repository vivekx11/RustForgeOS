//! Security sandbox with capability-based permissions

#![no_std]

extern crate alloc;

pub mod capability;
pub mod isolation;
pub mod syscall;

pub use capability::Capability;
