//! WebAssembly runtime

#![no_std]

extern crate alloc;

pub mod parser;
pub mod executor;
pub mod module;

pub use executor::WasmExecutor;
pub use module::WasmModule;
