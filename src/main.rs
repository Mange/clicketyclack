mod adapter;
mod audio_player;
use crate::adapter::{Adapter, ConnectError};
use audio_player::AudioPlayer;
use std::sync::mpsc;
use std::thread;

pub enum Event {
    KeyDown,
    KeyUp,
}

#[derive(Copy, Clone, Debug)]
pub enum Model {
    KailhBoxWhite,
}

fn main() -> Result<(), String> {
    let (sender, receiver) = mpsc::channel();

    let adapter = Adapter::connect(sender).map_err(|error| match error {
        ConnectError::FailedToOpenDisplay => "Failed to open display".to_string(),
        ConnectError::MissingXkbQueryExtension => "XkbQueryExtension is missing".to_string(),
        ConnectError::MissingXRecordExtension => "XRecord extension is missing".to_string(),
        ConnectError::XRecordAllocationFailed => "XRecord could not be allocated".to_string(),
        ConnectError::XRecordSetupFailed => "XRecord could not be setup".to_string(),
        ConnectError::XRecordEnableFailed => "XRecord could not be enabled".to_string(),
        ConnectError::UnknownError(code) => format!("Unknown error: {}", code),
    })?;

    // Record keys thread
    thread::spawn(move || {
        adapter.block();
    });

    // Audio setup
    let mut audio_player = AudioPlayer::new(Model::KailhBoxWhite, 0.3);

    for event in receiver {
        match event {
            Event::KeyDown => audio_player.play_key_down(),
            Event::KeyUp => audio_player.play_key_up(),
        }
    }

    Ok(())
}
