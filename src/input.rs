use rpassword::read_password;
use std::io::{self, Write};
use text_io::read;

// Prompt user input
pub fn get(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    let input: String = read!("{}\n");
    input
}

pub fn get_password(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    let password: String = read_password().unwrap();
    password
}
