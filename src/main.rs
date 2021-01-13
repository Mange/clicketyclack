mod adapter;
mod audio_player;
mod resources;
mod sounds;

use crate::adapter::Adapter;
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

    let adapter = Adapter::connect(sender).map_err(|error_code| {
        format!("Could not connect to adapter: Error code: {}", error_code)
    })?;

    // Record keys thread
    thread::spawn(move || {
        adapter.block();
    });

    // Audio setup
    let mut sounds = sounds::Sounds::load(Model::KailhBoxWhite);
    let volume = 0.3; // TODO: Make this changeable at runtime somehow
    let audio_player = AudioPlayer::new(volume);

    for event in receiver {
        match event {
            Event::KeyDown => audio_player.play_sound(sounds.next_down()),
            Event::KeyUp => audio_player.play_sound(sounds.next_up()),
        }
    }

    Ok(())
}
