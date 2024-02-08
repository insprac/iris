use std::io::{self, Write};

pub fn read(prompt: &str) -> String {
    let mut input = String::new();
    print!("[input] {}: ", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
