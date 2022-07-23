use dialoguer::FuzzySelect;
use dialoguer::theme::ColorfulTheme;
use librespot::core::session::Session;
use librespot::core::spotify_id::SpotifyId;
use librespot::metadata::{Metadata, Track, Album, Playlist};
use std::collections::HashMap;
use std::process;
use std::sync::mpsc::Sender;

use crate::command::Command;
use crate::command::CommandType;
use crate::fetch::Fetcher;
use crate::play::{Message, MessageType};

pub struct Invoker {
    session: Session,
    fetcher: Fetcher,
    transmitter: Sender<Message>,
}

impl Invoker {
    pub fn new(session: Session, fetcher: Fetcher, transmitter: Sender<Message>) -> Invoker {
        Invoker {
            session,
            fetcher,
            transmitter,
        }
    }

    pub async fn execute(&mut self, command: Command) -> Result<String, String> {
        match command.command_type {
            CommandType::Play => self.play(command.args).await,
            CommandType::Pause => (),
            CommandType::Stop => self.stop().await,
            CommandType::List => self.list(command.args),
            CommandType::Whoami => self.whoami(),
            CommandType::Quit => self.quit(),
            _ => self.unknown(),
        }

        Ok(String::from("Done execution"))
    }

    pub async fn play(&mut self, mut args: Vec<String>) {
        let first_arg = args.remove(0);
        let joined_args = args.join(" ");
        match first_arg.as_str() {
            "playlist" => {
                let playlists_map: HashMap<String, Playlist> = self.fetcher.playlists().clone(); // This looks like a boo boo
                self.select_and_play(&playlists_map, joined_args).await;
            },
            "album" => {
                let albums_map: HashMap<String, Album> = self.fetcher.albums().clone();
                self.select_and_play(&albums_map, joined_args).await;
            },
            _ => self.unknown(),
        };
    }

    pub async fn select_and_play<>(&mut self, track_collection_map: &HashMap<String, impl Tracks>, name:String) { //TODO: Naming is hard :(
        let keys = track_collection_map.keys().cloned().collect();
        let selection = match name.is_empty() {
            false => name,
            true => select_item(keys),
        };
        let selected_track_collection = track_collection_map.get(&selection);
        match selected_track_collection {
            None => println!("Not found :("),
            Some(p) => {
                self.send_to_player(p.tracks()).await;
            }
        }
    }

    pub async fn send_to_player(&mut self, track_ids: Vec<SpotifyId>) {
        let mut is_first_track = true;
        for track_spotify_id in track_ids {
            let track = Track::get(&self.session, track_spotify_id).await.unwrap();
            let message = Message {
                message_type: match is_first_track {
                    true => MessageType::StartPlaying,
                    false => MessageType::AddToQueue,
                },
                track: Some(track),
            };
            self.transmitter.send(message).unwrap();
            is_first_track = false;
        }
    }

    pub async fn stop(&mut self) {
        let message = Message {
            message_type: MessageType::StopPlaying,
            track: None,
        };
        self.transmitter.send(message).unwrap();
    }

    pub fn list(&self, args: Vec<String>) {
        // List all playlists
        let playlists = self.fetcher.playlists();
        for p in playlists.keys() {
            println!("{}", p);
        }
    }

    pub fn whoami(&self) {
        println!("Good question...");
    }

    pub fn quit(&self) {
        println!("Life without music is unthinkable.");
        process::exit(0);
    }

    pub fn unknown(&self) {
        println!("Huh?");
    }
}

pub fn select_item(items: Vec<String>) -> String {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_opt()
        .unwrap();
    match selection {
        Some(index) => items[index].to_owned(),
        None => String::new(),
    }
}

pub trait Tracks {
    fn tracks(&self) -> Vec<SpotifyId>;
}

impl Tracks for Album {
    fn tracks(&self) -> Vec<SpotifyId> {
        self.tracks.clone()
    }
}

impl Tracks for Playlist {
    fn tracks(&self) -> Vec<SpotifyId> {
        self.tracks.clone()
    }
}