use std::{
    collections::LinkedList,
    io::{self, Write},
    sync::mpsc::Receiver,
    thread::{self},
};

use indicatif::ProgressBar;
use librespot::playback::player::Player as LibrePlayer;
use librespot::{
    core::session::Session,
    metadata::Track,
    playback::{
        audio_backend,
        config::{AudioFormat, PlayerConfig},
        mixer::NoOpVolume,
        player::PlayerEvent,
    },
};

pub struct Player {}

impl Player {
    pub fn new(session: Session, receiver: Receiver<Message>) -> Player {
        let mut player = create_player(session);
        let mut track_queue: LinkedList<Track> = LinkedList::new();
        let mut events = player.get_player_event_channel();
        let mut spinner = ProgressBar::new_spinner();
        let _thread = thread::spawn(move || loop {
            match receiver.try_recv() {
                Ok(message) => match message {
                    Message::AddToQueue(track) => track_queue.push_back(track),
                    Message::StartPlaying(track) => {
                        track_queue.clear();
                        spinner = ProgressBar::new_spinner();
                        spinner.enable_steady_tick(120);
                        spinner.set_message(track.name);
                        player.load(track.id, true, 0);
                    }
                    Message::StopPlaying => {
                        player.stop();
                        track_queue.clear();
                        spinner.finish();
                        println!("Stopped");
                        print!(">> ");
                        io::stdout().flush().unwrap();
                    }
                    Message::Quit => {
                        break;
                    }
                },
                Err(_) => (),
            }

            match events.try_recv() {
                Ok(PlayerEvent::EndOfTrack { .. }) => {
                    if !track_queue.is_empty() {
                        let track = track_queue.pop_front().unwrap();
                        spinner.set_message(track.name);
                        player.load(track.id, true, 0);
                    }
                }
                Ok(_) => (),
                Err(_) => (),
            }
        });

        Player {}
    }
}

fn create_player(session: Session) -> LibrePlayer {
    let player_config = PlayerConfig::default();
    let audio_format = AudioFormat::default();
    let backend = audio_backend::find(None).unwrap();
    let result = LibrePlayer::new(player_config, session, Box::new(NoOpVolume), move || {
        backend(None, audio_format)
    });
    result.0
}

pub enum Message {
    StartPlaying(Track),
    StopPlaying,
    AddToQueue(Track),
    Quit,
}
