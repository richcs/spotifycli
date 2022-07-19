use librespot::core::keymaster::Token;
use reqwest::{Error, Client};

pub struct Fetcher {
    api_client: Client
}

impl Fetcher {
    pub fn new() -> Self {
        Self { api_client : reqwest::Client::new() }
    }

    pub async fn get_playlists(&self, token: Token) -> Result<(), Error> {
        let data = self.api_client
            .get("https://api.spotify.com/v1/me/playlists")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await?;
        
        println!("{:?}", data);
        return Ok(());
    }
}