use librespot::core::keymaster;
use librespot::playback::audio_backend;
use librespot::playback::config::{AudioFormat, PlayerConfig};
use librespot::playback::mixer::NoOpVolume;
use librespot::playback::player::Player;
use librespot::{
    core::{cache::Cache, config::SessionConfig, session::Session},
    discovery::Credentials,
};
use std::path::Path;
use std::process::exit;

mod command;
mod config;
mod fetch;
mod input;
mod invoke;

use command::Command;
use config as Config;
use fetch::Fetcher;
use input as Input;
use invoke::Invoker;

#[tokio::main]
async fn main() {
    let session = create_session().await;
    let token = keymaster::get_token(&session, Config::CLIENT_ID, Config::SCOPES)
        .await
        .unwrap();
    let mut fetcher = Fetcher::new();
    fetcher.fetch_playlists(token, &session).await.unwrap();
    let player = create_player(session);
    let invoker = Invoker::new(player, fetcher);
    loop {
        let input = Input::get(">>");
        if input.trim().is_empty() {
            continue;
        }
        
        let command = Command::new(input);
        invoker.execute(command);
    }
}

async fn create_session() -> Session {
    let path = Path::new(config::PATH_STRING);
    let cache = Cache::new(Some(path), None, None, None).ok();
    let credentials = get_credentials(&cache);
    let session_config = SessionConfig::default();
    let connect_result = Session::connect(session_config, credentials, cache, true).await;
    match connect_result {
        Result::Err(_) => {
            println!("Login Failed");
            exit(-1);
        }
        Result::Ok((session, _)) => {
            println!("Connected Successfully");
            session
        }
    }
}

fn create_player(session: Session) -> Player {
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();
    let backend = audio_backend::find(None).unwrap();
    let result = Player::new(player_config, session, Box::new(NoOpVolume), move || {
        backend(None, audio_format)
    });
    result.0
}

fn get_credentials(cache: &Option<Cache>) -> Credentials {
    let credential_path_string = Config::PATH_STRING.to_owned() + Config::CREDENTIALS_FILE;
    if !Path::new(credential_path_string.as_str()).exists() {
        return login_user_pass();
    }

    let saved_credentials = cache.as_ref().unwrap().credentials().unwrap();
    return saved_credentials;
}

fn login_user_pass() -> Credentials {
    println!("Login to Spotify");
    let username = Input::get("Enter Username:");
    let password = Input::get_password("Enter Password:");
    let credentials = Credentials::with_password(username, password);
    return credentials;
}
