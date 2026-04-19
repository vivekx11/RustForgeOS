//! Common types and utilities shared across RustForge OS

#![no_std]

pub mod error;
pub mod sync;
pub mod types;

pub use error::{Error, Result};
