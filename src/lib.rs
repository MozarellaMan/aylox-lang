#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate derive_is_enum_variant;
use ast_printer::AstPrinter;
use error::AyloxError;
use parser::Parser;
use scanner::Scanner;
use std::fs;

pub mod ast;
pub mod ast_printer;
pub mod error;
pub mod parser;
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
    let mut parser = Parser::new(scanner.scan_tokens());
    let expression = parser.parse();
    let mut printer = AstPrinter;

    match expression {
        Ok(expr) => println!("{}", printer.print(&expr)),
        Err(err) => println!("{}", err),
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
