use std::{
    collections::LinkedList,
    sync::mpsc::Receiver,
    thread::{self},
};

use indicatif::ProgressBar;
use librespot::core::session::Session;
use librespot::metadata::Track;
use librespot::playback::player::Player as LibrePlayer;
use librespot::playback::{
    audio_backend,
    config::{AudioFormat, PlayerConfig},
    mixer::NoOpVolume,
    player::PlayerEvent,
};

use crate::interact as Interact;

pub struct Player {}

impl Player {
    pub fn new(session: Session, receiver: Receiver<Message>) -> Player {
        let mut player = create_player(session);
        let mut track_queue: LinkedList<TrackData> = LinkedList::new();
        let mut events = player.get_player_event_channel();
        let mut spinner = ProgressBar::new_spinner();
        let builder = thread::Builder::new().name("track_player".into());
        let _thread = builder.spawn(move || loop {
            match receiver.try_recv() {
                Ok(message) => match message {
                    Message::AddToQueue(track_data) => track_queue.push_back(track_data),
                    Message::StartPlaying(track_data) => {
                        track_queue.clear();
                        spinner = Interact::start_player_spinner();
                        spinner.set_message(track_data.track.name + " - " + &track_data.artist.to_string());
                        player.load(track_data.track.id, true, 0);
                    }
                    Message::StopPlaying => {
                        player.stop();
                        track_queue.clear();
                        Interact::stop_player_spinner(&spinner);
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
                        let track_data = track_queue.pop_front().unwrap();
                        spinner.set_message(track_data.track.name + " - " + &track_data.artist.to_string());
                        player.load(track_data.track.id, true, 0);
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
    StartPlaying(TrackData),
    StopPlaying,
    AddToQueue(TrackData),
    Quit,
}

pub struct TrackData {
    pub track: Track,
    pub artist: String,
}