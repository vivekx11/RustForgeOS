//! Stack-based Virtual Machine for ForgeScript bytecode

use alloc::vec::Vec;
use crate::bytecode::*;

const STACK_SIZE: usize = 256;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    locals: Vec<Value>,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(STACK_SIZE),
            locals: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<Option<Value>, String> {
        loop {
            let opcode = self.read_opcode()?;
            
            match opcode {
                OpCode::Push => {
                    let idx = self.read_byte()? as usize;
                    let value = self.chunk.constants[idx].clone();
                    self.push(value);
                }
                OpCode::Pop => {
                    self.pop()?;
                }
                OpCode::Dup => {
                    let value = self.peek()?.clone();
                    self.push(value);
                }
                OpCode::Swap => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    self.push(a);
                    self.push(b);
                }
                OpCode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.push(Value::Int(x + y)),
                        (Value::Float(x), Value::Float(y)) => self.push(Value::Float(x + y)),
                        _ => return Err("Type error in addition".into()),
                    }
                }
                OpCode::Sub => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.push(Value::Int(x - y)),
                        (Value::Float(x), Value::Float(y)) => self.push(Value::Float(x - y)),
                        _ => return Err("Type error in subtraction".into()),
                    }
                }
                OpCode::Mul => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.push(Value::Int(x * y)),
                        (Value::Float(x), Value::Float(y)) => self.push(Value::Float(x * y)),
                        _ => return Err("Type error in multiplication".into()),
                    }
                }
                OpCode::Div => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => {
                            if y == 0 {
                                return Err("Division by zero".into());
                            }
                            self.push(Value::Int(x / y))
                        }
                        (Value::Float(x), Value::Float(y)) => self.push(Value::Float(x / y)),
                        _ => return Err("Type error in division".into()),
                    }
                }
                OpCode::Mod => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.push(Value::Int(x % y)),
                        _ => return Err("Type error in modulo".into()),
                    }
                }
                OpCode::Neg => {
                    let a = self.pop()?;
                    match a {
                        Value::Int(x) => self.push(Value::Int(-x)),
                        Value::Float(x) => self.push(Value::Float(-x)),
                        _ => return Err("Type error in negation".into()),
                    }
                }
                OpCode::Eq => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(Value::Bool(a == b));
                }
                OpCode::NotEq => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(Value::Bool(a != b));
                }
                OpCode::Lt => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.push(Value::Bool(x < y)),
                        (Value::Float(x), Value::Float(y)) => self.push(Value::Bool(x < y)),
                        _ => return Err("Type error in comparison".into()),
                    }
                }
                OpCode::LtEq => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.push(Value::Bool(x <= y)),
                        (Value::Float(x), Value::Float(y)) => self.push(Value::Bool(x <= y)),
                        _ => return Err("Type error in comparison".into()),
                    }
                }
                OpCode::Gt => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.push(Value::Bool(x > y)),
                        (Value::Float(x), Value::Float(y)) => self.push(Value::Bool(x > y)),
                        _ => return Err("Type error in comparison".into()),
                    }
                }
                OpCode::GtEq => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Int(x), Value::Int(y)) => self.push(Value::Bool(x >= y)),
                        (Value::Float(x), Value::Float(y)) => self.push(Value::Bool(x >= y)),
                        _ => return Err("Type error in comparison".into()),
                    }
                }
                OpCode::And => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(Value::Bool(a.is_truthy() && b.is_truthy()));
                }
                OpCode::Or => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(Value::Bool(a.is_truthy() || b.is_truthy()));
                }
                OpCode::Not => {
                    let a = self.pop()?;
                    self.push(Value::Bool(!a.is_truthy()));
                }
                OpCode::Jump => {
                    let addr = self.read_u16()? as usize;
                    self.ip = addr;
                }
                OpCode::JumpIfFalse => {
                    let addr = self.read_u16()? as usize;
                    let condition = self.peek()?;
                    if !condition.is_truthy() {
                        self.ip = addr;
                    }
                }
                OpCode::Call => {
                    return Err("Function calls not yet implemented".into());
                }
                OpCode::Return => {
                    let value = self.pop()?;
                    return Ok(Some(value));
                }
                OpCode::LoadLocal => {
                    let idx = self.read_byte()? as usize;
                    if idx >= self.locals.len() {
                        return Err("Local variable index out of bounds".into());
                    }
                    let value = self.locals[idx].clone();
                    self.push(value);
                }
                OpCode::StoreLocal => {
                    let idx = self.read_byte()? as usize;
                    let value = self.pop()?;
                    if idx >= self.locals.len() {
                        self.locals.resize(idx + 1, Value::Null);
                    }
                    self.locals[idx] = value;
                }
                OpCode::LoadGlobal => {
                    return Err("Global variables not yet implemented".into());
                }
                OpCode::StoreGlobal => {
                    return Err("Global variables not yet implemented".into());
                }
                OpCode::NewArray => {
                    let len = self.pop()?.as_int().ok_or("Array length must be int")?;
                    let mut elements = Vec::new();
                    for _ in 0..len {
                        elements.push(self.pop()?);
                    }
                    elements.reverse();
                    self.push(Value::Array(elements));
                }
                OpCode::ArrayGet => {
                    let index = self.pop()?.as_int().ok_or("Array index must be int")?;
                    let array = self.pop()?;
                    match array {
                        Value::Array(ref arr) => {
                            if index < 0 || index >= arr.len() as i64 {
                                return Err("Array index out of bounds".into());
                            }
                            self.push(arr[index as usize].clone());
                        }
                        _ => return Err("Can only index arrays".into()),
                    }
                }
                OpCode::ArraySet => {
                    let value = self.pop()?;
                    let index = self.pop()?.as_int().ok_or("Array index must be int")?;
                    let mut array = self.pop()?;
                    match array {
                        Value::Array(ref mut arr) => {
                            if index < 0 || index >= arr.len() as i64 {
                                return Err("Array index out of bounds".into());
                            }
                            arr[index as usize] = value;
                            self.push(array);
                        }
                        _ => return Err("Can only index arrays".into()),
                    }
                }
                OpCode::ArrayLen => {
                    let array = self.pop()?;
                    match array {
                        Value::Array(arr) => self.push(Value::Int(arr.len() as i64)),
                        _ => return Err("Can only get length of arrays".into()),
                    }
                }
                OpCode::Halt => {
                    return Ok(None);
                }
            }
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or_else(|| "Stack underflow".into())
    }

    fn peek(&self) -> Result<&Value, String> {
        self.stack.last().ok_or_else(|| "Stack is empty".into())
    }

    fn read_byte(&mut self) -> Result<u8, String> {
        if self.ip >= self.chunk.code.len() {
            return Err("Unexpected end of bytecode".into());
        }
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        Ok(byte)
    }

    fn read_u16(&mut self) -> Result<u16, String> {
        let high = self.read_byte()? as u16;
        let low = self.read_byte()? as u16;
        Ok((high << 8) | low)
    }

    fn read_opcode(&mut self) -> Result<OpCode, String> {
        let byte = self.read_byte()?;
        // SAFETY: We trust that the compiler only generates valid opcodes
        Ok(unsafe { core::mem::transmute(byte) })
    }
}
