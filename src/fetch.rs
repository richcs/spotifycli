use std::collections::HashMap;

use crate::config as Config;
use librespot::{
    core::{
        keymaster::{self, Token},
        session::Session,
        spotify_id::{SpotifyId, SpotifyIdError},
    },
    metadata::{Album, Metadata, Playlist},
};

pub struct Fetcher {
    playlists: HashMap<String, Playlist>,
}

impl Fetcher {
    pub async fn new(session: &Session) -> Result<Fetcher, Box<dyn std::error::Error>> {
        let api_client = reqwest::Client::new();
        let mut playlists: HashMap<String, Playlist> = HashMap::new();
        let token = fetch_token(session).await;

        // Get user's playlists
        let data = api_client
            .get("https://api.spotify.com/v1/me/playlists?fields=items(id)")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await?
            .text()
            .await?;

        let playlist = serde_json::from_str::<JsonPlaylists>(&data.to_owned())?;
        for p in playlist.items {
            let playlist = fetch_individual::<Playlist>(p.id, session).await.unwrap();
            playlists.insert(playlist.name.clone(), playlist);
        }

        // Get user's albums
        let data = api_client
            .get("https://api.spotify.com/v1/me/albums?limit=10&fields=items(album(id))")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await?
            .text()
            .await?;

        let album_ids = serde_json::from_str::<JsonAlbums>(&data.to_owned())?;
        let fetcher = Fetcher { playlists };

        Ok(fetcher)
    }

    pub fn playlists(&self) -> &HashMap<String, Playlist> {
        let playlists = &self.playlists;
        playlists
    }
}

pub async fn fetch_token(session: &Session) -> Token {
    let token = keymaster::get_token(session, Config::CLIENT_ID, Config::SCOPES)
        .await
        .unwrap();
    token
}

pub async fn fetch_individual<T: Metadata>(id: String, session: &Session) -> Result<T, SpotifyIdError> {
    let spotify_id_result = SpotifyId::from_base62(&id);
    match spotify_id_result {
        Ok(spotify_id) => {
            let item = T::get(session, spotify_id).await.unwrap();
            Ok(item)
        }
        Err(SpotifyIdError) => panic!(),
    }
}

#[derive(serde::Deserialize)]
pub struct JsonPlaylists {
    items: Vec<JsonPlaylist>,
}

#[derive(serde::Deserialize, Clone)]
pub struct JsonPlaylist {
    pub id: String,
}

#[derive(serde::Deserialize)]
pub struct JsonAlbums {
    items: Vec<JsonWhatIsThis>,
}

#[derive(serde::Deserialize, Clone)]
pub struct JsonWhatIsThis { //TODO: Hmm what to name...
    pub album: JsonAlbum,
}

#[derive(serde::Deserialize, Clone)]
pub struct JsonAlbum {
    pub id: String,
}
