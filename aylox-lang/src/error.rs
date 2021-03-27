use thiserror::Error;

use crate::ast::Expr;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Parsing Failed.")]
    Generic,
    #[error("[line {line}], found '{lexeme}'. {msg}")]
    UnexpectedToken {
        line: usize,
        lexeme: String,
        msg: String,
    },
    #[error("[line {line}] at end of line: {msg}.")]
    UnexpectedEof { line: usize, msg: String },
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("[line {line}] Unexpected token, found '{found}'.")]
    UnexpectedToken { line: usize, found: String },
    #[error("[line {line}] Unterminated string.")]
    UnterminatedString { line: usize },
}
#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("[line {line}] '{lexeme}' operands must be {expected}.")]
    InvalidOperand {
        lexeme: String,
        expected: String,
        line: usize,
    },
    #[error("[line {line}] '{lexeme}' not available for {expression:?}.")]
    InvalidOperator {
        lexeme: String,
        expression: Expr,
        line: usize,
    },
    #[error("[line {line}] Variable '{lexeme}' is undefined.")]
    UndefinedVariable { lexeme: String, line: usize },
    #[error("[line {line}] Tried to use nil variable '{lexeme}'")]
    NilAccess { lexeme: String, line: usize },
    #[error("Runtime environment does not exist. This is likely an interpreter error.")]
    EnvironmentError,
}

#[derive(Error, Debug)]
pub enum AyloxError {
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),
    #[error("Error: {0}.")]
    GenericError(String),
    #[error("Syntax error: {0}")]
    SyntaxError(#[from] SyntaxError),
    #[error("Parsing failed")]
    ParserError(#[from] ParserError),
    #[error("Runtime error: {0}")]
    RuntimeError(#[from] RuntimeError),
}
