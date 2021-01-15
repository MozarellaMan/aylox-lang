use error::AyloxError;
use scanner::scan_tokens;
use std::fs;

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
    for token in scan_tokens(contents).iter() {
        println!("{}", token)
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
