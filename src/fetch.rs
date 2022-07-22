use std::collections::HashMap;

use crate::config as Config;
use librespot::{
    core::{
        keymaster::{self, Token},
        session::Session,
        spotify_id::{SpotifyId, SpotifyIdError},
    },
    metadata::{Metadata, Playlist},
};
use reqwest::Client;

pub struct Fetcher {
    api_client: Client,
    playlists: HashMap<String, Playlist>,
}

impl Fetcher {
    pub fn new() -> Fetcher {
        let fetcher = Fetcher {
            api_client: reqwest::Client::new(),
            playlists: HashMap::new(),
        };
        fetcher
    }

    pub async fn fetch_playlists(
        &mut self,
        session: &Session,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let token = fetch_token(session).await;
        let data = self
            .api_client
            .get("https://api.spotify.com/v1/me/playlists?fields=items(id)")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await?
            .text()
            .await?;

        let playlist_ids = serde_json::from_str::<PlaylistIds>(&data.to_owned())?;
        for p in playlist_ids.items {
            let playlist = self.fetch_playlist(p, session).await.unwrap();
            self.store_playlist(playlist);
        }

        return Ok(());
    }

    pub async fn fetch_playlist(
        &self,
        playlist_id: PlaylistId,
        session: &Session,
    ) -> Result<Playlist, SpotifyIdError> {
        let playlist_id = SpotifyId::from_base62(&playlist_id.id);
        match playlist_id {
            Ok(spotify_id) => {
                let playlist = Playlist::get(session, spotify_id).await.unwrap();
                Ok(playlist)
            }
            Err(SpotifyIdError) => panic!(),
        }
    }

    pub fn store_playlist(&mut self, playlist: Playlist) {
        self.playlists.insert(playlist.name.clone(), playlist);
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

#[derive(serde::Deserialize)]
pub struct PlaylistIds {
    items: Vec<PlaylistId>,
}

#[derive(serde::Deserialize, Clone)]
pub struct PlaylistId {
    pub id: String,
}
