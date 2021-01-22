use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("At line {line}, found '{lexeme}'. {msg}")]
    UnexpectedToken {
        line: usize,
        lexeme: String,
        msg: String,
    },
    #[error("{line} at end. {msg}")]
    UnexpectedEof { line: usize, msg: String },
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Unexpected token at line {line}, found '{found}'")]
    UnexpectedToken { line: usize, found: String },
    #[error("Unterminated string.")]
    UnterminatedString,
}
#[derive(Error, Debug)]
pub enum AyloxError {
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),
    #[error("Error: {0}.")]
    GenericError(String),
    #[error("Syntax error: {0}")]
    SyntaxError(#[from] SyntaxError),
    #[error("Parser error: {0}")]
    ParserError(#[from] ParserError),
}
