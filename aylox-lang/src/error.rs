use thiserror::Error;

use crate::ast::{AloxObject, Expr};
// TODO Newtype for lines in code

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
    #[error("[line {line}] Function cannot have more than 255 arguments.")]
    FunctionArgumentLength { line: usize },
    #[error("[line {line}] Function cannot have more than 255 parameters.")]
    FunctionParameterLength { line: usize },
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("[line {line}] Unexpected token, found '{found}'.")]
    UnexpectedToken { line: usize, found: String },
    #[error("[line {line}] Unterminated string.")]
    UnterminatedString { line: usize },
}
#[derive(Error, Debug)]
pub enum RuntimeException {
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
    #[error("Could not find an execution branch. This is likely an interpreter error.")]
    ControlFlowError,
    #[error("[line {line:?}] Expected a value here (bool, nil, string or number) at {lexeme:?}")]
    ValueMissing {
        line: Option<usize>,
        lexeme: Option<String>,
    },
    #[error("[line {line}] Expected a function at '{lexeme}")]
    ExpectedFunction { lexeme: String, line: usize },
    #[error("Returning {obj:?}")]
    Return { obj: AloxObject },
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
    RuntimeError(#[from] RuntimeException),
}

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("[line {line}] Can't read local variable in its own initializer at '{lexeme}")]
    ReadInOwnInitializer { lexeme: String, line: usize },
}
