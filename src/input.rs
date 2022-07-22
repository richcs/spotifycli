use rpassword::read_password;
use std::io::{self, BufRead, Write};
use text_io::read;

pub fn get() -> String {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    input.trim().to_string()
}

// Prompt user input
pub fn get_with_prompt(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    let input: String = read!("{}\n");
    input.trim().to_string()
}

pub fn get_password(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    let password: String = read_password().unwrap();
    password
}
