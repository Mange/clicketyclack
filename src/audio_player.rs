use crate::sounds::Sound;
use rodio::{DeviceSinkBuilder, Source};

pub struct AudioPlayer {
    sink: rodio::MixerDeviceSink,
    volume: f32,
}

impl AudioPlayer {
    pub fn build(volume: f32) -> Result<Self, rodio::DeviceSinkError> {
        let mut sink = DeviceSinkBuilder::open_default_sink()?;
        sink.log_on_drop(false);

        Ok(AudioPlayer { sink, volume })
    }

    pub fn play_sound(&self, sound: Sound) {
        self.sink.mixer().add(sound.amplify(self.volume));
    }
}
