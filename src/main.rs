mod adapter;
mod audio_player;
mod sounds;

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
    let mut sounds = sounds::Sounds::load(Model::KailhBoxWhite);
    let volume = 0.3; // TODO: Make this changeable at runtime somehow
    let audio_player = AudioPlayer::new();

    for event in receiver {
        match event {
            Event::KeyDown => audio_player.play_sound(sounds.next_down(), volume),
            Event::KeyUp => audio_player.play_sound(sounds.next_up(), volume),
        }
    }

    Ok(())
}
