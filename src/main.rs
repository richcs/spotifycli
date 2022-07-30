use librespot::core::{cache::Cache, config::SessionConfig, session::Session};
use librespot::discovery::Credentials;
use std::path::PathBuf;
use std::process::exit;
use std::sync::mpsc::{self, Receiver, Sender};

mod command;
mod config;
mod fetch;
mod interact;
mod invoke;
mod model;
mod play;

use command::Command;
use config as Config;
use fetch::Fetcher;
use interact as Interact;
use invoke::Invoker;
use play::{Message, Player};

use crate::interact::println;

#[tokio::main]
async fn main() {
    let session = create_session().await;
    let spinner = Interact::start_session_spinner();
    let fetcher = Fetcher::new(&session).await.unwrap();
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let _player = Player::new(session.clone(), rx);
    let mut invoker = Invoker::new(session, fetcher, tx);
    Interact::stop_session_spinner(spinner);
    loop {
        let input = Interact::get();
        if input.is_empty() {
            continue;
        }

        let command = Command::new(input);
        let execution_result = invoker.execute(command).await;
        match execution_result {
            Ok(_) => (),
            Err(_) => (),
        }
    }
}

async fn create_session() -> Session {
    let path = get_credentials_path();
    let cache = Cache::new(Some(path), None, None, None).ok();
    let credentials = get_credentials(&cache);
    let session_config = SessionConfig::default();
    let connect_result = Session::connect(session_config, credentials, cache, true).await;
    match connect_result {
        Result::Err(_) => {
            println("Login Failed");
            exit(-1);
        }
        Result::Ok((session, _)) => session,
    }
}

fn get_credentials_path() -> PathBuf {
    let mut local_data_path = dirs::data_local_dir().unwrap();
    local_data_path.push(Config::APP_NAME);
    local_data_path
}

fn get_credentials(cache: &Option<Cache>) -> Credentials {
    let saved_credentials = cache.as_ref().unwrap().credentials();
    match saved_credentials {
        Some(sc) => sc,
        None => login_user_pass(),
    }
}

fn login_user_pass() -> Credentials {
    println("Login to Spotify");
    let username = Interact::get_username();
    let password = Interact::get_password();
    let credentials = Credentials::with_password(username, password);
    return credentials;
}
