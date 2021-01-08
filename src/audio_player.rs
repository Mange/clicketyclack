use crate::Model;
use rodio::{source::Amplify, source::Buffered, Decoder, OutputStream, OutputStreamHandle, Source};
use std::fs::File;
use std::io::BufReader;

type AudioSource = Buffered<Amplify<Decoder<BufReader<File>>>>;

pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    audio: AudioRepository,
    up_cursor: usize,
    down_cursor: usize,
}

pub struct AudioRepository {
    down: Vec<AudioSource>,
    up: Vec<AudioSource>,
}

impl AudioRepository {
    fn load(model: Model, volume: f32) -> AudioRepository {
        match model {
            Model::KailhBoxWhite => AudioRepository {
                down: load_audio_files("down", "kailh_white", 5, volume),
                up: load_audio_files("up", "kailh_white", 5, volume),
            },
        }
    }
}

fn load_audio_files(direction: &str, name: &str, count: i32, volume: f32) -> Vec<AudioSource> {
    assert!(count > 0);

    (1..=count)
        .into_iter()
        .map(|i| {
            load_audio_file(
                &format!(
                    "resources/{name}/{direction}{n}.wav",
                    name = name,
                    direction = direction,
                    n = i
                ),
                volume,
            )
        })
        .collect()
}

fn load_audio_file(path: &str, volume: f32) -> AudioSource {
    let sound_file = File::open(path).unwrap();
    rodio::Decoder::new(BufReader::new(sound_file))
        .unwrap()
        .amplify(volume)
        .buffered()
}

impl AudioPlayer {
    pub fn new(model: Model, volume: f32) -> Self {
        let audio = AudioRepository::load(model, volume);
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

        AudioPlayer {
            _stream: stream,
            stream_handle,
            audio,
            up_cursor: 0,
            down_cursor: 0,
        }
    }

    pub fn play_key_down(&mut self) {
        let source = self.audio.down[self.down_cursor].clone();
        self.down_cursor = (self.down_cursor + 1) % self.audio.down.len();

        self.stream_handle.play_raw(source.convert_samples()).ok();
    }

    pub fn play_key_up(&mut self) {
        let source = self.audio.up[self.up_cursor].clone();
        self.up_cursor = (self.up_cursor + 1) % self.audio.up.len();

        self.stream_handle.play_raw(source.convert_samples()).ok();
    }
}
