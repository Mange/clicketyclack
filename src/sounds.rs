use crate::resources;
use crate::Model;
use rodio::{source::Buffered, Decoder, Source};
use std::io::Cursor;

pub type Sound = Buffered<Decoder<Cursor<&'static [u8]>>>;

pub struct Sounds {
    down: SoundSet,
    up: SoundSet,
}

impl Sounds {
    pub fn load(model: Model) -> Sounds {
        match model {
            Model::KailhBoxWhite => Sounds {
                down: SoundSet::load(resources::load_kailh_white_down()),
                up: SoundSet::load(resources::load_kailh_white_up()),
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
    pub fn load(resource: Vec<&'static [u8]>) -> SoundSet {
        let sounds: Vec<Sound> = resource.into_iter().map(decode_wav).collect();
        assert!(!sounds.is_empty());

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

fn decode_wav(bytes: &'static [u8]) -> Sound {
    rodio::Decoder::new_wav(Cursor::new(bytes))
        .unwrap()
        .buffered()
}
