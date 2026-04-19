//! Bytecode instruction set for ForgeScript VM

use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum OpCode {
    // Stack operations
    Push,
    Pop,
    Dup,
    Swap,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,

    // Comparison
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,

    // Logical
    And,
    Or,
    Not,

    // Control flow
    Jump,
    JumpIfFalse,
    Call,
    Return,

    // Variables
    LoadLocal,
    StoreLocal,
    LoadGlobal,
    StoreGlobal,

    // Arrays
    NewArray,
    ArrayGet,
    ArraySet,
    ArrayLen,

    // Halt
    Halt,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(alloc::vec::Vec<Value>),
    Null,
}

impl Value {
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::Null => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub code: alloc::vec::Vec<u8>,
    pub constants: alloc::vec::Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: alloc::vec::Vec::new(),
            constants: alloc::vec::Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn write_opcode(&mut self, opcode: OpCode) {
        self.write(opcode as u8);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
