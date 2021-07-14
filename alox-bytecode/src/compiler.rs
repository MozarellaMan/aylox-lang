use crate::scanner::Scanner;


pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source);
    scanner.scan();
}