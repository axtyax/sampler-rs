use rodio::{source::Source, Sink, OutputStream, OutputStreamHandle};
use super::sound::{Sound};

pub struct PlaybackHandle {
    sink: Sink
}

impl PlaybackHandle {
    pub fn new(stream_handle: &OutputStreamHandle) -> Self {
        Self {
            sink: Sink::try_new(&stream_handle).unwrap()
        }
    }

    pub fn load_sound(&mut self, sound: Sound) -> () {
        self.sink.append(sound.into_iter());
    }

    pub fn sleep_until_end(&self) -> () {
        self.sink.sleep_until_end();
    }

}

pub struct SoundPlayer {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl SoundPlayer {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            stream,
            stream_handle,
        }
    }

    pub fn play_sound(&self, sound: Sound) -> PlaybackHandle {
        let mut playback_handle = PlaybackHandle::new(&self.stream_handle);
        playback_handle.load_sound(sound);
        playback_handle
    }
}

