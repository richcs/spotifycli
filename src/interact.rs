use std::io::{self, Write};
use console::style;
use dialoguer::{Password, FuzzySelect, theme::ColorfulTheme};
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
    spinner.finish_with_message("Ready! (type 'help' for commands)")
}

pub fn start_player_spinner() -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner().template("{spinner:.blue} {msg:.blue}")
    );
    spinner.enable_steady_tick(120);
    spinner
}

pub fn stop_player_spinner(spinner: &ProgressBar) {
    spinner.finish();
    println("Stopped");
    print_prompt();
}

pub fn select_item(items: Vec<&String>) -> String {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_opt()
        .unwrap();
    match selection {
        Some(index) => items[index].to_owned(),
        None => String::new(),
    }
}

pub fn print_help() {
    println("Available Commands:");
    println("play playlist/album         Select and play a playlist/album");
    println("play playlist/album <name>  Play a playlist/album with name <name>");
    println("ls playlist/album           Print list of available playlists/albums");
    println("whoami                      Print your username");
    println("help                        Print list of available commands");
    println("quit                        Exit program");
}
