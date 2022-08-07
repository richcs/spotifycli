use console::{Key, Term};
use futures::executor::block_on;
use librespot::core::session::Session;
use librespot::core::spotify_id::SpotifyId;
use librespot::metadata::{Album, Artist, Metadata, Playlist, Track};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::{process, thread};

use crate::command::Command;
use crate::command::CommandType;
use crate::fetch::Fetcher;
use crate::interact::println;
use crate::interact::{self as Interact, print_help};
use crate::play::{Message, TrackData};

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
            CommandType::Help => self.help(),
            CommandType::Quit => self.quit(),
            _ => self.unknown(),
        }

        Ok(String::from("Done execution"))
    }

    pub async fn play(&mut self, mut args: Vec<String>) {
        if args.is_empty() {
            self.unknown();
            return;
        }
        let first_arg = args.remove(0);
        let mut shuffle = false;
        match args.get(0) {
            None => (),
            Some(arg) => {
                if arg == "shuffle" {
                    shuffle = true;
                    args.remove(0);
                }
            }
        }
        let joined_args = args.join(" ");
        match first_arg.as_str() {
            // TODO: Merge these somehow (or maybe not...)
            "playlist" => {
                let track_collection =
                    select_track_collection(self.fetcher.playlists(), joined_args);
                match track_collection {
                    None => {
                        println("Not found");
                        return;
                    }
                    Some(tc) => {
                        play_track_collection(tc, shuffle, &self.session, &self.transmitter).await;
                    }
                }
            }
            "album" => {
                let track_collection = select_track_collection(self.fetcher.albums(), joined_args);
                match track_collection {
                    None => {
                        println("Not found");
                        return;
                    }
                    Some(tc) => {
                        play_track_collection(tc, false, &self.session, &self.transmitter).await;
                    }
                }
            }
            _ => {
                self.unknown();
                return;
            }
        };

        // Wait for user input to stop playing music
        let stdout = Term::stdout();
        loop {
            let key_result = stdout.read_key();
            match key_result {
                Ok(Key::Unknown | Key::UnknownEscSeq(_)) => (),
                _ => {
                    self.stop().await;
                    break;
                }
            }
        }
    }

    pub async fn stop(&mut self) {
        let message = Message::StopPlaying;
        self.transmitter.send(message).unwrap();
    }

    pub fn list(&self, mut args: Vec<String>) {
        if args.is_empty() {
            self.unknown();
            return;
        }
        let first_arg = args.remove(0);
        match first_arg.as_str() {
            "playlist" | "playlists" => {
                for p in self.fetcher.playlists().keys() {
                    println(p);
                }
            }
            "album" | "albums" => {
                for a in self.fetcher.albums().keys() {
                    println(a);
                }
            }
            _ => self.unknown(),
        };
    }

    pub fn whoami(&self) {
        println(&self.session.username());
    }

    pub fn help(&self) {
        print_help();
    }

    pub fn quit(&self) {
        let message = Message::Quit;
        self.transmitter.send(message).unwrap();
        println("Come back soon!");
        process::exit(0);
    }

    pub fn unknown(&self) {
        println("Huh?");
    }
}

async fn play_track_collection(
    tc: &impl TrackCollection,
    shuffle: bool,
    session: &Session,
    transmitter: &Sender<Message>,
) {
    let tracks = match shuffle {
        false => tc.tracks(),
        true => tc.shuffled_tracks(),
    };
    let session = session.clone();
    let transmitter = transmitter.clone();
    thread::spawn(move || block_on(send_to_player(tracks, session, transmitter)));
    // This works?
}

fn select_track_collection(
    track_collection_map: &HashMap<String, impl TrackCollection>,
    name: String,
) -> Option<&impl TrackCollection> {
    let keys: Vec<&String> = track_collection_map.keys().collect();
    let selection = match name.is_empty() {
        false => {
            let mut matching_key = String::from("");
            for key in keys.iter() {
                if key.contains(&name) {
                    matching_key = key.to_string();
                    break;
                }
            }
            matching_key
        }
        true => Interact::select_item(keys),
    };
    let selected_track_collection = track_collection_map.get(&selection);
    selected_track_collection
}

async fn send_to_player(track_ids: Vec<SpotifyId>, session: Session, transmitter: Sender<Message>) {
    let mut is_first_track = true;
    for track_spotify_id in track_ids {
        let track_result = Track::get(&session, track_spotify_id).await;
        match track_result {
            Ok(track) => {
                let artist_result = Artist::get(&session, track.artists[0]).await;
                match artist_result {
                    Ok(artist) => {
                        let message = create_message(track, artist, is_first_track);
                        transmitter.send(message).unwrap_or_else(|err| {
                            eprintln!("Problem sending track to player: {}", err);
                        });
                    }
                    Err(_) => (),
                }
            }
            Err(_) => (), // TODO: How should I handle?
        }
        is_first_track = false;
    }
}

fn create_message(track: Track, artist: Artist, is_first_track: bool) -> Message {
    let track_data = TrackData {
        track,
        artist: artist.name,
    };
    let message = match is_first_track {
        true => Message::StartPlaying(track_data),
        false => Message::AddToQueue(track_data),
    };
    message
}

pub trait TrackCollection {
    fn tracks(&self) -> Vec<SpotifyId>;
    fn shuffled_tracks(&self) -> Vec<SpotifyId>;
    fn name(&self) -> String;
}

impl TrackCollection for Album {
    fn tracks(&self) -> Vec<SpotifyId> {
        self.tracks.clone()
    }

    fn shuffled_tracks(&self) -> Vec<SpotifyId> {
        let mut rng = thread_rng();
        let mut shuffled_tracks = self.tracks.clone();
        shuffled_tracks.shuffle(&mut rng);
        shuffled_tracks
    }

    fn name(&self) -> String {
        self.name.to_string()
    }
}

impl TrackCollection for Playlist {
    fn tracks(&self) -> Vec<SpotifyId> {
        self.tracks.clone()
    }

    fn shuffled_tracks(&self) -> Vec<SpotifyId> {
        let mut rng = thread_rng();
        let mut shuffled_tracks = self.tracks.clone();
        shuffled_tracks.shuffle(&mut rng);
        shuffled_tracks
    }

    fn name(&self) -> String {
        self.name.to_string()
    }
}
