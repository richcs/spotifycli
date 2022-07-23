use librespot::core::session::Session;
use librespot::metadata::{Metadata, Track};
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

    pub async fn play(&mut self, args: Vec<String>) {
        // Play a playlist
        let joined_args = args.join(" ");
        let playlist_name = joined_args;
        let playlist_tracks = self.fetcher.playlists().get(&playlist_name);
        match playlist_tracks {
            None => println!("Not found :("),
            Some(p) => {
                let mut is_first_track = true;
                for track_spotify_id in &p.tracks {
                    let track = Track::get(&self.session, *track_spotify_id).await.unwrap();
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
