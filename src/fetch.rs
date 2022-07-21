use std::collections::HashMap;

use librespot::{
    core::{
        keymaster::Token,
        session::Session,
        spotify_id::{self, SpotifyId, SpotifyIdError},
    },
    metadata::{Metadata, Playlist, Track},
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
        token: Token,
        session: &Session,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = self
            .api_client
            .get("https://api.spotify.com/v1/me/playlists")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await?
            .text()
            .await?;

        let playlists = serde_json::from_str::<Playlists>(&data.to_owned())?;
        for p in playlists.items {
            let playlist_id = SpotifyId::from_base62(&p.id);
            match playlist_id {
                Ok(spotify_id) => {
                    let playlist = Playlist::get(session, spotify_id).await.unwrap();
                    let playlist_name = playlist.name.to_owned();
                    self.playlists.insert(playlist_name, playlist);
                }
                Err(SpotifyIdError) => panic!(),
            }
            break;
        }

        return Ok(());
    }

    pub fn playlists(&self) -> &HashMap<String, Playlist> {
        let playlists = &self.playlists;
        playlists
    }
}

#[derive(serde::Deserialize)]
pub struct Playlists {
    items: Vec<PlaylistId>,
}

#[derive(serde::Deserialize, Clone)]
pub struct PlaylistId {
    pub id: String,
}
