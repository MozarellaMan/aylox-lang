pub fn scan_tokens(input: &str) -> Vec<&str> {
    input.split(' ').collect::<_>()
}
