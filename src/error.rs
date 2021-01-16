use thiserror::Error;
#[derive(Error, Debug)]
pub enum AyloxError {
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),
    #[error("Error: {0}.")]
    GenericError(String),
    #[error("Syntax Error: Unexpected token at line {line}, found '{found}'")]
    Syntax { line: usize, found: String },
    #[error("Syntax Error: Unterminated string.")]
    UnterminatedString,
}
