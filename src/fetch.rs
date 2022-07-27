use std::collections::HashMap;

use crate::config as Config;
use librespot_core::{
    keymaster::{self, Token},
    session::Session,
    spotify_id::{SpotifyId, SpotifyIdError},
};
use librespot_metadata::{Album, Metadata, Playlist};

pub struct Fetcher {
    playlists: HashMap<String, Playlist>,
    albums: HashMap<String, Album>,
}

impl Fetcher {
    pub async fn new(session: &Session) -> Result<Fetcher, Box<dyn std::error::Error>> {
        let api_client = reqwest::Client::new();
        let mut playlists: HashMap<String, Playlist> = HashMap::new();
        let mut albums: HashMap<String, Album> = HashMap::new();
        let token = fetch_token(session).await;

        // Get user's playlists
        let playlists_endpoint =
            String::from("https://api.spotify.com/v1/me/playlists?fields=items(id)");
        let playlists_json = request(&api_client, playlists_endpoint, &token)
            .await
            .unwrap();
        let fetched_playlists = serde_json::from_str::<PlaylistsModel>(playlists_json.as_str())?;
        for p in fetched_playlists.items {
            let playlist = fetch_individual::<Playlist>(p.id, session).await.unwrap();
            playlists.insert(playlist.name.to_owned(), playlist);
        }

        // Get user's albums
        let albums_endpoint =
            String::from("https://api.spotify.com/v1/me/albums?fields=items(album(id))");
        let albums_json = request(&api_client, albums_endpoint, &token).await.unwrap();
        let fetched_albums = serde_json::from_str::<AlbumsModel>(albums_json.as_str())?;
        for album_wrapper in fetched_albums.items {
            let album = fetch_individual::<Album>(album_wrapper.album.id, session)
                .await
                .unwrap();
            albums.insert(album.name.to_owned(), album);
        }

        let fetcher = Fetcher { playlists, albums };
        Ok(fetcher)
    }

    pub fn playlists(&self) -> &HashMap<String, Playlist> {
        &self.playlists
    }

    pub fn albums(&self) -> &HashMap<String, Album> {
        &self.albums
    }
}

pub async fn fetch_token(session: &Session) -> Token {
    let token = keymaster::get_token(session, Config::CLIENT_ID, Config::SCOPES)
        .await
        .unwrap();
    token
}

pub async fn request(
    api_client: &reqwest::Client,
    endpoint: String,
    token: &Token,
) -> Result<String, reqwest::Error> {
    let data = api_client
        .get(endpoint)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()
        .await?
        .text()
        .await?;
    Ok(data)
}

pub async fn fetch_individual<T: Metadata>(
    id: String,
    session: &Session,
) -> Result<T, SpotifyIdError> {
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
pub struct PlaylistsModel {
    items: Vec<PlaylistModel>,
}

#[derive(serde::Deserialize, Clone)]
pub struct PlaylistModel {
    pub id: String,
}

#[derive(serde::Deserialize)]
pub struct AlbumsModel {
    items: Vec<AlbumWrapperModel>,
}

#[derive(serde::Deserialize, Clone)]
pub struct AlbumWrapperModel {
    // What is this...
    pub album: AlbumModel,
}

#[derive(serde::Deserialize, Clone)]
pub struct AlbumModel {
    pub id: String,
}
