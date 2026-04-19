//! Distributed systems layer with Raft consensus

#![no_std]

extern crate alloc;

pub mod raft;
pub mod kv;
pub mod node;

pub use raft::RaftNode;
pub use kv::KeyValueStore;
