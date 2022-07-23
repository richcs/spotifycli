use std::io::{self, Write};
use dialoguer::Password;
use text_io::read;

pub fn get() -> String {
    let input: String = read!("{}\n");
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
    let password = Password::new().with_prompt(prompt)
    .interact();
    password.unwrap()
}
