mod adapter;
mod audio_player;
mod resources;
mod sounds;

use audio_player::AudioPlayer;
use clap::Parser;
use std::sync::mpsc;

use crate::adapter::Adapter;

#[derive(Clone, clap::ValueEnum)]
#[value(rename_all = "PascalCase")]
pub enum Model {
    KailhBoxWhite,
}

#[derive(Parser)]
#[command(version, about)]
/// Play sounds of mechanical keyboards when you type, for that extra clicky feeling inside your
/// headset without waking up your family in the night.
struct Options {
    #[arg(
        value_enum,
        long = "model",
        short = 'm',
        default_value = "KailhBoxWhite",
        value_name = "NAME"
    )]
    /// The switch model you want to emulate.
    model: Model,

    #[arg(
        long = "volume",
        short = 'l',
        alias = "level",
        default_value = "1.0",
        value_name = "FLOAT",
        value_parser = volume_parser
    )]
    /// Fixed volume level for the duration of the program (between >0.0 and 2.0).
    volume: f32,
}

pub enum Event {
    KeyDown,
    KeyUp,
}

fn volume_parser(value: &str) -> Result<f32, String> {
    let volume: f32 = value
        .parse()
        .map_err(|_| format!("`{value}` isn't a valid float"))?;

    if volume <= 0.0 || volume > 2.0 {
        return Err(format!("`{value}` is not in range 0 < volume ≤ 2.0"));
    }
    Ok(volume)
}

fn main() -> Result<(), String> {
    let options = Options::parse();
    let (sender, receiver) = mpsc::channel();

    let adapter = Adapter::connect(sender).map_err(|error_code| {
        format!("Could not connect to adapter: Error code: {}", error_code)
    })?;

    // Audio setup
    let mut sounds = sounds::Sounds::load(options.model);
    let audio_player = AudioPlayer::build(options.volume)
        .map_err(|error| format!("Could not open default audio device: {error}"))?;

    // Record keys thread
    std::thread::spawn(move || {
        adapter.block();
    });

    for event in receiver {
        match event {
            Event::KeyDown => audio_player.play_sound(sounds.next_down()),
            Event::KeyUp => audio_player.play_sound(sounds.next_up()),
        }
    }

    Ok(())
}
