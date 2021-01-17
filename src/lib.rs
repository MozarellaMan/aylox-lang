#[macro_use]
extern crate derive_new;
use error::AyloxError;
use scanner::Scanner;
use std::fs;

pub mod ast;
pub mod error;
pub mod prompt;
pub mod scanner;
pub mod token;

pub fn run_file(path: &str) -> Result<(), AyloxError> {
    let contents = fs::read_to_string(path)?;
    run(&contents)?;
    Ok(())
}

pub fn run(contents: &str) -> Result<(), AyloxError> {
    let mut scanner = Scanner::new(contents);
    for token in scanner.scan_tokens().iter() {
        println!("{:?}", token)
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
