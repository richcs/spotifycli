#[derive(serde::Deserialize)]
pub struct PlaylistsModel {
    pub items: Vec<PlaylistModel>,
}

#[derive(serde::Deserialize, Clone)]
pub struct PlaylistModel {
    pub id: String,
}

#[derive(serde::Deserialize)]
pub struct AlbumsModel {
    pub items: Vec<AlbumWrapperModel>,
}

#[derive(serde::Deserialize, Clone)]
pub struct AlbumWrapperModel {
    // What is this...
    pub album: AlbumModel,
}

#[derive(serde::Deserialize, Clone)]
pub struct AlbumModel {
    pub id: String,
    pub name: String,
    pub artists: Vec<ArtistModel>,
}

#[derive(serde::Deserialize, Clone)]
pub struct ArtistModel {
    pub id: String,
    pub name: String,
}
