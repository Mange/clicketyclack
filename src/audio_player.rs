use crate::sounds::Sound;
use rodio::{OutputStream, OutputStreamHandle, Source};

pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

        AudioPlayer {
            _stream: stream,
            stream_handle,
        }
    }

    pub fn play_sound(&self, sound: Sound, volume: f32) {
        self.stream_handle
            .play_raw(sound.amplify(volume).convert_samples())
            .ok();
    }
}
