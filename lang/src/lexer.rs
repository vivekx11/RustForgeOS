//! Lexer for ForgeScript - tokenizes source code

use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    #[token("let")]
    Let,
    #[token("fn")]
    Fn,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Types
    #[token("int")]
    Int,
    #[token("float")]
    Float,
    #[token("string")]
    String,
    #[token("bool")]
    Bool,
    #[token("array")]
    Array,

    // Literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    IntLiteral(i64),
    
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    FloatLiteral(f64),
    
    #[regex(r#""([^"\\]|\\["\\bnfrt])*""#, |lex| lex.slice().to_string())]
    StringLiteral(alloc::string::String),

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(alloc::string::String),

    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    
    #[token("==")]
    EqEq,
    #[token("!=")]
    NotEq,
    #[token("<")]
    Lt,
    #[token("<=")]
    LtEq,
    #[token(">")]
    Gt,
    #[token(">=")]
    GtEq,
    
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,
    
    #[token("=")]
    Eq,

    // Delimiters
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,

    // Whitespace and comments
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip)]
    #[error]
    Error,
}

pub struct Lexer<'source> {
    inner: logos::Lexer<'source, Token>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            inner: Token::lexer(source),
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
