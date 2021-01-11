use crate::Model;
use rodio::{source::Buffered, Decoder, Source};
use std::fs::File;
use std::io::BufReader;

pub type Sound = Buffered<Decoder<BufReader<File>>>;

pub struct Sounds {
    down: SoundSet,
    up: SoundSet,
}

impl Sounds {
    pub fn load(model: Model) -> Sounds {
        match model {
            Model::KailhBoxWhite => Sounds {
                down: SoundSet::load("down", "kailh_white", 5),
                up: SoundSet::load("up", "kailh_white", 5),
            },
        }
    }

    pub fn next_up(&mut self) -> Sound {
        self.up.next()
    }

    pub fn next_down(&mut self) -> Sound {
        self.down.next()
    }
}

pub struct SoundSet {
    current_index: usize,
    sounds: Vec<Sound>,
}

impl SoundSet {
    pub fn load(direction: &str, name: &str, count: i32) -> SoundSet {
        assert!(count > 0);

        let sounds = (1..=count)
            .into_iter()
            .map(|i| {
                load_audio_file(&format!(
                    "resources/{name}/{direction}{n}.wav",
                    name = name,
                    direction = direction,
                    n = i
                ))
            })
            .collect();

        SoundSet {
            current_index: 0,
            sounds,
        }
    }

    pub fn next(&mut self) -> Sound {
        let source = self.sounds[self.current_index].clone();
        self.current_index = (self.current_index + 1) % self.sounds.len();
        source
    }
}

fn load_audio_file(path: &str) -> Sound {
    let sound_file = File::open(path).unwrap();
    rodio::Decoder::new(BufReader::new(sound_file))
        .unwrap()
        .buffered()
}
