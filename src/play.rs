use std::{
    collections::LinkedList,
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

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

pub struct Player {
    thread: JoinHandle<()>,
}

impl Player {
    pub fn new(session: Session, receiver: Receiver<Message>) -> Player {
        let mut player = create_player(session);
        let mut track_queue: LinkedList<Track> = LinkedList::new();
        let mut events = player.get_player_event_channel();
        let thread = thread::spawn(move || loop {
            match receiver.try_recv() {
                Ok(message) => match message.message_type {
                    MessageType::AddToQueue => track_queue.push_back(message.track.unwrap()),
                    MessageType::StartPlaying => {
                        let track = message.track.unwrap();
                        player.load(track.id, true, 0);
                    }
                    MessageType::StopPlaying => {
                        player.stop();
                        track_queue.clear();
                    }
                },
                Err(_) => (),
            }

            match events.try_recv() {
                Ok(PlayerEvent::EndOfTrack { .. }) => {
                    if !track_queue.is_empty() {
                        let track = track_queue.pop_front().unwrap();
                        player.load(track.id, true, 0);
                    }
                }
                Ok(_) => (),
                Err(_) => (),
            }
        });

        Player { thread }
    }

    // pub fn play_next_in_queue(&self) {
    //     let track = track_queue.pop_front().unwrap();
    //     self.player.load(track.id, true, 0);
    // }
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

pub struct Message {
    pub message_type: MessageType,
    pub track: Option<Track>,
}

pub enum MessageType {
    StartPlaying,
    StopPlaying,
    AddToQueue,
}
