use std::process;

use librespot::{core::session::Session, playback::player::Player};

use crate::command::Command;
use crate::command::CommandType;
use crate::fetch::Fetcher;

pub struct Invoker {
    player: Player,
    fetcher: Fetcher,
}

impl Invoker {
    pub fn new(my_player: Player, my_fetcher: Fetcher) -> Invoker {
        let invoker = Invoker {
            player: my_player,
            fetcher: my_fetcher,
        };
        invoker
    }

    pub fn execute(&self, command: Command) {
        match command.command_type {
            CommandType::Play => self.play(command.args),
            CommandType::List => self.list(command.args),
            CommandType::Quit => quit(),
            _ => println!("Huh?"),
        }
    }

    pub fn play(&self, args: Vec<String>) {
        // Play a playlist
        let joined_args = args.join(" ");
        let playlist_name = joined_args;
        let playlist_tracks = self.fetcher.playlists().get(&playlist_name);
        match playlist_tracks {
            None => println!("Not found :("),
            Some(p) => {
                for track in &p.tracks {
                    println!("{}", track.id);
                }
            }
        }
    }

    pub fn list(&self, args: Vec<String>) {
        // List all playlists
        let playlists = self.fetcher.playlists();
        for p in playlists.keys() {
            println!("{}", p);
        }
    }
}

fn quit() {
    println!("Quiting Spotify, goodbye cruel world...");
    process::exit(0);
}
