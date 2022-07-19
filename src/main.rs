use std::io::{self, Write};
use std::path::Path;
use std::process::exit;
use librespot::core::keymaster;
use text_io::read;
use rpassword::read_password;
use librespot::{core::{config::SessionConfig, session::Session, cache::Cache}, discovery::Credentials};

mod config;
mod fetcher;

const SCOPES: &str =
    "streaming,user-read-playback-state,user-modify-playback-state,user-read-currently-playing,playlist-read-private";

#[tokio::main]
async fn main() {
    let path = Path::new(config::PATH_STRING);
    let cache = Cache::new(Some(path), None, None, None).ok();
    let credentials = get_credentials(&cache);
    let session_config = SessionConfig::default();
    let connect_result = Session::connect(session_config, credentials, cache, true).await;
    match connect_result {
        Result::Err(_) => { 
            println!("Login Failed");
            exit(0);
        },
        Result::Ok((session, _)) => {
            let token = keymaster::get_token(&session, config::CLIENT_ID, SCOPES).await.unwrap();
            let spot_fetcher = fetcher::Fetcher::new();
            let playlists = spot_fetcher.get_playlists(token).await.unwrap();
            println!("Connected Successfully");
        }
    }

    // Retrieve data from spotify

    // Start accepting commands
}

fn get_credentials(cache : &Option<Cache>) -> Credentials {
    let credential_path_string = config::PATH_STRING.to_owned() + config::CREDENTIALS_FILE;
    if !Path::new(credential_path_string.as_str()).exists() {
        return login_user_pass();
    }
    
    let saved_credentials = cache.as_ref().unwrap().credentials().unwrap();
    return saved_credentials;
}

fn login_user_pass() -> Credentials {
    println!("Login to Spotify");
    
    print!("Enter Username: ");
    io::stdout().flush().unwrap();
    let username : String = read!("{}\n");
    print!("Enter Password: ");
    io::stdout().flush().unwrap();
    let password : String = read_password().unwrap();
    let credentials = Credentials::with_password(username, password);
    return credentials;
}