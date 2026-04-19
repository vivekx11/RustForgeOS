//! ForgeScript - Custom programming language for RustForge OS

#![no_std]

extern crate alloc;

pub mod ast;
pub mod bytecode;
pub mod compiler;
pub mod lexer;
pub mod parser;
pub mod vm;

pub use lexer::Lexer;
pub use parser::Parser;
pub use compiler::Compiler;
pub use vm::VM;
