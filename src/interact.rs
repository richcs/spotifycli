use std::io::{self, Write};
use console::style;
use dialoguer::Password;
use indicatif::{ProgressBar, ProgressStyle};
use text_io::read;

pub fn get() -> String {
    print!("{} ", style(">>").green());
    io::stdout().flush().unwrap();
    let input: String = read!("{}\n");
    input.trim().to_string()
}

pub fn get_username() -> String {
    print!("{} ", "Enter Username:");
    io::stdout().flush().unwrap();
    let input: String = read!("{}\n");
    input.trim().to_string()
}

pub fn get_password() -> String {
    let password = Password::new().with_prompt("Enter Password")
    .interact();
    password.unwrap()
}

pub fn println(text: &str) {
    println!("{}", style(text).green())
}

pub fn print_prompt() {
    print!("{} ", style(">>").green());
    io::stdout().flush().unwrap();
}

pub fn start_session_spinner() -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner().template("{spinner:.green} {msg:.green}")
    );
    spinner.set_message("Starting session...");
    spinner.enable_steady_tick(120);
    spinner
}

pub fn stop_session_spinner(spinner: ProgressBar) {
    spinner.finish_with_message("Ready!")
}
