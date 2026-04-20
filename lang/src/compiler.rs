//! Bytecode compiler for ForgeScript AST
// compiler 
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use crate::ast::*;
use crate::bytecode::*;

pub struct Compiler {
    chunk: Chunk,
    locals: Vec<String>,
    globals: BTreeMap<String, usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            locals: Vec::new(),
            globals: BTreeMap::new(),
        }
    }

    pub fn compile(mut self, program: Program) -> Result<Chunk, String> {
        for statement in program.statements {
            self.compile_statement(statement)?;
        }
        self.chunk.write_opcode(OpCode::Halt);
        Ok(self.chunk)
    }

    fn compile_statement(&mut self, stmt: Statement) -> Result<(), String> {
        match stmt {
            Statement::Let { name, value, .. } => {
                self.compile_expression(value)?;
                let idx = self.locals.len();
                self.locals.push(name);
                self.chunk.write_opcode(OpCode::StoreLocal);
                self.chunk.write(idx as u8);
            }
            Statement::Function { .. } => {
                // TODO: Implement function compilation
                return Err("Function compilation not yet implemented".into());
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    self.compile_expression(e)?;
                } else {
                    let idx = self.chunk.add_constant(Value::Null);
                    self.chunk.write_opcode(OpCode::Push);
                    self.chunk.write(idx as u8);
                }
                self.chunk.write_opcode(OpCode::Return);
            }
            Statement::Expression(expr) => {
                self.compile_expression(expr)?;
                self.chunk.write_opcode(OpCode::Pop);
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.compile_expression(condition)?;
                
                // Jump if false placeholder
                self.chunk.write_opcode(OpCode::JumpIfFalse);
                let jump_if_false_addr = self.chunk.code.len();
                self.chunk.write(0);
                self.chunk.write(0);

                // Then branch
                for stmt in then_branch {
                    self.compile_statement(stmt)?;
                }

                if let Some(else_stmts) = else_branch {
                    // Jump over else
                    self.chunk.write_opcode(OpCode::Jump);
                    let jump_addr = self.chunk.code.len();
                    self.chunk.write(0);
                    self.chunk.write(0);

                    // Patch jump if false
                    let else_addr = self.chunk.code.len();
                    self.chunk.code[jump_if_false_addr] = (else_addr >> 8) as u8;
                    self.chunk.code[jump_if_false_addr + 1] = else_addr as u8;

                    // Else branch
                    for stmt in else_stmts {
                        self.compile_statement(stmt)?;
                    }

                    // Patch jump over else
                    let end_addr = self.chunk.code.len();
                    self.chunk.code[jump_addr] = (end_addr >> 8) as u8;
                    self.chunk.code[jump_addr + 1] = end_addr as u8;
                } else {
                    // Patch jump if false
                    let end_addr = self.chunk.code.len();
                    self.chunk.code[jump_if_false_addr] = (end_addr >> 8) as u8;
                    self.chunk.code[jump_if_false_addr + 1] = end_addr as u8;
                }
            }
            Statement::While { condition, body } => {
                let loop_start = self.chunk.code.len();
                
                self.compile_expression(condition)?;
                
                self.chunk.write_opcode(OpCode::JumpIfFalse);
                let jump_addr = self.chunk.code.len();
                self.chunk.write(0);
                self.chunk.write(0);

                for stmt in body {
                    self.compile_statement(stmt)?;
                }

                // Jump back to loop start
                self.chunk.write_opcode(OpCode::Jump);
                self.chunk.write((loop_start >> 8) as u8);
                self.chunk.write(loop_start as u8);

                // Patch exit jump
                let end_addr = self.chunk.code.len();
                self.chunk.code[jump_addr] = (end_addr >> 8) as u8;
                self.chunk.code[jump_addr + 1] = end_addr as u8;
            }
            Statement::For { .. } => {
                return Err("For loop compilation not yet implemented".into());
            }
        }
        Ok(())
    }

    fn compile_expression(&mut self, expr: Expression) -> Result<(), String> {
        match expr {
            Expression::IntLiteral(n) => {
                let idx = self.chunk.add_constant(Value::Int(n));
                self.chunk.write_opcode(OpCode::Push);
                self.chunk.write(idx as u8);
            }
            Expression::FloatLiteral(f) => {
                let idx = self.chunk.add_constant(Value::Float(f));
                self.chunk.write_opcode(OpCode::Push);
                self.chunk.write(idx as u8);
            }
            Expression::StringLiteral(s) => {
                let idx = self.chunk.add_constant(Value::String(s));
                self.chunk.write_opcode(OpCode::Push);
                self.chunk.write(idx as u8);
            }
            Expression::BoolLiteral(b) => {
                let idx = self.chunk.add_constant(Value::Bool(b));
                self.chunk.write_opcode(OpCode::Push);
                self.chunk.write(idx as u8);
            }
            Expression::ArrayLiteral(elements) => {
                let len = elements.len();
                for elem in elements {
                    self.compile_expression(elem)?;
                }
                let len_idx = self.chunk.add_constant(Value::Int(len as i64));
                self.chunk.write_opcode(OpCode::Push);
                self.chunk.write(len_idx as u8);
                self.chunk.write_opcode(OpCode::NewArray);
            }
            Expression::Identifier(name) => {
                if let Some(idx) = self.locals.iter().position(|n| n == &name) {
                    self.chunk.write_opcode(OpCode::LoadLocal);
                    self.chunk.write(idx as u8);
                } else {
                    return Err(alloc::format!("Undefined variable: {}", name));
                }
            }
            Expression::Binary { left, op, right } => {
                self.compile_expression(*left)?;
                self.compile_expression(*right)?;
                
                let opcode = match op {
                    BinaryOp::Add => OpCode::Add,
                    BinaryOp::Sub => OpCode::Sub,
                    BinaryOp::Mul => OpCode::Mul,
                    BinaryOp::Div => OpCode::Div,
                    BinaryOp::Mod => OpCode::Mod,
                    BinaryOp::Eq => OpCode::Eq,
                    BinaryOp::NotEq => OpCode::NotEq,
                    BinaryOp::Lt => OpCode::Lt,
                    BinaryOp::LtEq => OpCode::LtEq,
                    BinaryOp::Gt => OpCode::Gt,
                    BinaryOp::GtEq => OpCode::GtEq,
                    BinaryOp::And => OpCode::And,
                    BinaryOp::Or => OpCode::Or,
                };
                
                self.chunk.write_opcode(opcode);
            }
            Expression::Unary { op, operand } => {
                self.compile_expression(*operand)?;
                
                let opcode = match op {
                    UnaryOp::Neg => OpCode::Neg,
                    UnaryOp::Not => OpCode::Not,
                };
                
                self.chunk.write_opcode(opcode);
            }
            Expression::Call { function, args } => {
                for arg in args {
                    self.compile_expression(arg)?;
                }
                // TODO: Implement function calls
                return Err("Function calls not yet implemented".into());
            }
            Expression::Index { array, index } => {
                self.compile_expression(*array)?;
                self.compile_expression(*index)?;
                self.chunk.write_opcode(OpCode::ArrayGet);
            }
        }
        Ok(())
    }
}
