mod adapter;
mod audio_player;
mod resources;
mod sounds;

use crate::adapter::Adapter;
use audio_player::AudioPlayer;
use std::sync::mpsc;
use std::thread;
use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Copy, Clone, Debug)]
    pub enum Model {
        KailhBoxWhite,
    }
}

#[derive(StructOpt)]
#[structopt(
    about = "Play sounds of mechanical keyboards when you type, for that extra clicky feeling inside your headset without waking up your family in the night.",
    setting = structopt::clap::AppSettings::ColoredHelp
)]
struct Options {
    #[structopt(
        long = "model",
        short = "m",
        default_value = "KailhBoxWhite",
        possible_values = &Model::variants(),
        value_name = "NAME",
    )]
    model: Model,

    #[structopt(
        long = "volume",
        short = "l",
        alias = "level",
        default_value = "1.0",
        value_name = "FLOAT"
    )]
    volume: f32,
}

pub enum Event {
    KeyDown,
    KeyUp,
}

fn main() -> Result<(), String> {
    let options = Options::from_args();
    let (sender, receiver) = mpsc::channel();

    let adapter = Adapter::connect(sender).map_err(|error_code| {
        format!("Could not connect to adapter: Error code: {}", error_code)
    })?;

    // Record keys thread
    thread::spawn(move || {
        adapter.block();
    });

    // Audio setup
    let mut sounds = sounds::Sounds::load(options.model);
    let audio_player = AudioPlayer::new(options.volume);

    for event in receiver {
        match event {
            Event::KeyDown => audio_player.play_sound(sounds.next_down()),
            Event::KeyUp => audio_player.play_sound(sounds.next_up()),
        }
    }

    Ok(())
}
