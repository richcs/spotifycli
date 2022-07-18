use std::io::{self, Write, Read, stdin};
use std::path::Path;
use text_io::read;
use rpassword::read_password;
use librespot::{core::{config::SessionConfig, session::Session, cache::Cache}, discovery::Credentials};

static PATH_STRING:&str = "C:/ProgramData/spotifycli"; // TODO: CHANGE THIS
static CREDENTIALS_FILE:&str = "/credentials.json";

#[tokio::main]
async fn main() {
    login_user_pass();
    let path = Path::new(PATH_STRING);
    let cache = Cache::new(Some(path), None, None, None).ok();
    let credentials = get_credentials(&cache);
    let session_config = SessionConfig::default();
    let (session, _) = Session::connect(session_config, credentials, cache, true).await.unwrap();
    println!("Connected Successfully");

    // Start accepting commands
}

fn get_credentials(cache : &Option<Cache>) -> Credentials {
    let credential_path_string = PATH_STRING.to_owned() + CREDENTIALS_FILE;
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
    println!("This is your username: {}", username);
    print!("Enter Password: ");
    io::stdout().flush().unwrap();
    let password : String = read_password().unwrap();
    let credentials = Credentials::with_password(username, password);
    return credentials;
}