//! Recursive descent parser for ForgeScript

use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::String;
use crate::ast::*;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        
        Ok(Program { statements })
    }

    fn statement(&mut self) -> Result<Statement, String> {
        match self.peek() {
            Some(Token::Let) => self.let_statement(),
            Some(Token::Fn) => self.function_statement(),
            Some(Token::Return) => self.return_statement(),
            Some(Token::If) => self.if_statement(),
            Some(Token::While) => self.while_statement(),
            Some(Token::For) => self.for_statement(),
            _ => self.expression_statement(),
        }
    }

    fn let_statement(&mut self) -> Result<Statement, String> {
        self.consume(Token::Let)?;
        
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n,
            _ => return Err("Expected identifier after 'let'".into()),
        };

        let type_annotation = if matches!(self.peek(), Some(Token::Colon)) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume(Token::Eq)?;
        let value = self.expression()?;
        self.consume(Token::Semicolon)?;

        Ok(Statement::Let {
            name,
            type_annotation,
            value,
        })
    }

    fn function_statement(&mut self) -> Result<Statement, String> {
        self.consume(Token::Fn)?;
        
        let name = match self.advance() {
            Some(Token::Identifier(n)) => n,
            _ => return Err("Expected function name".into()),
        };

        self.consume(Token::LParen)?;
        let mut params = Vec::new();
        
        while !matches!(self.peek(), Some(Token::RParen)) {
            let param_name = match self.advance() {
                Some(Token::Identifier(n)) => n,
                _ => return Err("Expected parameter name".into()),
            };
            
            self.consume(Token::Colon)?;
            let param_type = self.parse_type()?;
            params.push((param_name, param_type));
            
            if matches!(self.peek(), Some(Token::Comma)) {
                self.advance();
            }
        }
        
        self.consume(Token::RParen)?;
        
        let return_type = if matches!(self.peek(), Some(Token::Arrow)) {
            self.advance();
            self.parse_type()?
        } else {
            Type::Void
        };

        self.consume(Token::LBrace)?;
        let mut body = Vec::new();
        
        while !matches!(self.peek(), Some(Token::RBrace)) {
            body.push(self.statement()?);
        }
        
        self.consume(Token::RBrace)?;

        Ok(Statement::Function {
            name,
            params,
            return_type,
            body,
        })
    }

    fn return_statement(&mut self) -> Result<Statement, String> {
        self.consume(Token::Return)?;
        
        let value = if matches!(self.peek(), Some(Token::Semicolon)) {
            None
        } else {
            Some(self.expression()?)
        };
        
        self.consume(Token::Semicolon)?;
        Ok(Statement::Return(value))
    }

    fn if_statement(&mut self) -> Result<Statement, String> {
        self.consume(Token::If)?;
        let condition = self.expression()?;
        
        self.consume(Token::LBrace)?;
        let mut then_branch = Vec::new();
        while !matches!(self.peek(), Some(Token::RBrace)) {
            then_branch.push(self.statement()?);
        }
        self.consume(Token::RBrace)?;

        let else_branch = if matches!(self.peek(), Some(Token::Else)) {
            self.advance();
            self.consume(Token::LBrace)?;
            let mut else_stmts = Vec::new();
            while !matches!(self.peek(), Some(Token::RBrace)) {
                else_stmts.push(self.statement()?);
            }
            self.consume(Token::RBrace)?;
            Some(else_stmts)
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Statement, String> {
        self.consume(Token::While)?;
        let condition = self.expression()?;
        
        self.consume(Token::LBrace)?;
        let mut body = Vec::new();
        while !matches!(self.peek(), Some(Token::RBrace)) {
            body.push(self.statement()?);
        }
        self.consume(Token::RBrace)?;

        Ok(Statement::While { condition, body })
    }

    fn for_statement(&mut self) -> Result<Statement, String> {
        self.consume(Token::For)?;
        self.consume(Token::LParen)?;
        
        let init = Box::new(self.statement()?);
        let condition = self.expression()?;
        self.consume(Token::Semicolon)?;
        let update = Box::new(self.expression_statement()?);
        
        self.consume(Token::RParen)?;
        self.consume(Token::LBrace)?;
        
        let mut body = Vec::new();
        while !matches!(self.peek(), Some(Token::RBrace)) {
            body.push(self.statement()?);
        }
        self.consume(Token::RBrace)?;

        Ok(Statement::For {
            init,
            condition,
            update,
            body,
        })
    }

    fn expression_statement(&mut self) -> Result<Statement, String> {
        let expr = self.expression()?;
        self.consume(Token::Semicolon)?;
        Ok(Statement::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expression, String> {
        self.logical_or()
    }

    fn logical_or(&mut self) -> Result<Expression, String> {
        let mut left = self.logical_and()?;

        while matches!(self.peek(), Some(Token::Or)) {
            self.advance();
            let right = self.logical_and()?;
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn logical_and(&mut self) -> Result<Expression, String> {
        let mut left = self.equality()?;

        while matches!(self.peek(), Some(Token::And)) {
            self.advance();
            let right = self.equality()?;
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn equality(&mut self) -> Result<Expression, String> {
        let mut left = self.comparison()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::EqEq => BinaryOp::Eq,
                Token::NotEq => BinaryOp::NotEq,
                _ => break,
            };
            self.advance();
            let right = self.comparison()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expression, String> {
        let mut left = self.term()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Lt => BinaryOp::Lt,
                Token::LtEq => BinaryOp::LtEq,
                Token::Gt => BinaryOp::Gt,
                Token::GtEq => BinaryOp::GtEq,
                _ => break,
            };
            self.advance();
            let right = self.term()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Expression, String> {
        let mut left = self.factor()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.factor()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<Expression, String> {
        let mut left = self.unary()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::Percent => BinaryOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.unary()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<Expression, String> {
        match self.peek() {
            Some(Token::Minus) => {
                self.advance();
                Ok(Expression::Unary {
                    op: UnaryOp::Neg,
                    operand: Box::new(self.unary()?),
                })
            }
            Some(Token::Not) => {
                self.advance();
                Ok(Expression::Unary {
                    op: UnaryOp::Not,
                    operand: Box::new(self.unary()?),
                })
            }
            _ => self.postfix(),
        }
    }

    fn postfix(&mut self) -> Result<Expression, String> {
        let mut expr = self.primary()?;

        loop {
            match self.peek() {
                Some(Token::LParen) => {
                    self.advance();
                    let mut args = Vec::new();
                    
                    while !matches!(self.peek(), Some(Token::RParen)) {
                        args.push(self.expression()?);
                        if matches!(self.peek(), Some(Token::Comma)) {
                            self.advance();
                        }
                    }
                    
                    self.consume(Token::RParen)?;
                    
                    if let Expression::Identifier(name) = expr {
                        expr = Expression::Call {
                            function: name,
                            args,
                        };
                    } else {
                        return Err("Can only call functions".into());
                    }
                }
                Some(Token::LBracket) => {
                    self.advance();
                    let index = self.expression()?;
                    self.consume(Token::RBracket)?;
                    expr = Expression::Index {
                        array: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expression, String> {
        match self.advance() {
            Some(Token::IntLiteral(n)) => Ok(Expression::IntLiteral(n)),
            Some(Token::FloatLiteral(f)) => Ok(Expression::FloatLiteral(f)),
            Some(Token::StringLiteral(s)) => Ok(Expression::StringLiteral(s)),
            Some(Token::True) => Ok(Expression::BoolLiteral(true)),
            Some(Token::False) => Ok(Expression::BoolLiteral(false)),
            Some(Token::Identifier(name)) => Ok(Expression::Identifier(name)),
            Some(Token::LParen) => {
                let expr = self.expression()?;
                self.consume(Token::RParen)?;
                Ok(expr)
            }
            Some(Token::LBracket) => {
                let mut elements = Vec::new();
                while !matches!(self.peek(), Some(Token::RBracket)) {
                    elements.push(self.expression()?);
                    if matches!(self.peek(), Some(Token::Comma)) {
                        self.advance();
                    }
                }
                self.consume(Token::RBracket)?;
                Ok(Expression::ArrayLiteral(elements))
            }
            _ => Err("Unexpected token in expression".into()),
        }
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        match self.advance() {
            Some(Token::Int) => Ok(Type::Int),
            Some(Token::Float) => Ok(Type::Float),
            Some(Token::String) => Ok(Type::String),
            Some(Token::Bool) => Ok(Type::Bool),
            Some(Token::Array) => {
                self.consume(Token::Lt)?;
                let inner = self.parse_type()?;
                self.consume(Token::Gt)?;
                Ok(Type::Array(Box::new(inner)))
            }
            _ => Err("Expected type".into()),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            self.current += 1;
            self.tokens.get(self.current - 1).cloned()
        } else {
            None
        }
    }

    fn consume(&mut self, expected: Token) -> Result<(), String> {
        if self.peek() == Some(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(alloc::format!("Expected {:?}", expected))
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
