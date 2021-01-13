use crate::sounds::Sound;
use rodio::{OutputStream, OutputStreamHandle, Source};

pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    volume: f32,
}

impl AudioPlayer {
    pub fn new(volume: f32) -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

        AudioPlayer {
            _stream: stream,
            stream_handle,
            volume,
        }
    }

    pub fn play_sound(&self, sound: Sound) {
        self.stream_handle
            .play_raw(sound.amplify(self.volume).convert_samples())
            .ok();
    }
}
