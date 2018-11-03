use sdl2;
use sdl2::mixer::{DEFAULT_CHANNELS, INIT_MP3, INIT_FLAC, INIT_MOD, INIT_OGG, AUDIO_S16LSB};

use std::path::Path;

pub struct Sounds<'a> {
    pub ping: sdl2::mixer::Music<'a>,
    pub pong: sdl2::mixer::Music<'a>
}

impl<'a> Sounds<'a> {
    pub fn new() -> Sounds<'a> {
        let frequency = 44_100;
        let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
        let channels = DEFAULT_CHANNELS; // Stereo
        let chunk_size = 1_024;

        sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
        let _mixer_context = sdl2::mixer::init(
            INIT_MP3 | INIT_FLAC | INIT_MOD | INIT_OGG
        ).unwrap();

        sdl2::mixer::allocate_channels(4);

        let ping_file = Path::new("./sounds/ping.wav");
        let pong_file = Path::new("./sounds/pong.wav");

        Sounds {
            ping: sdl2::mixer::Music::from_file(ping_file).unwrap(),
            pong: sdl2::mixer::Music::from_file(pong_file).unwrap(),
        }
    }

    pub fn pong(&self) -> () { self.play(&self.pong); }
    pub fn ping(&self) -> () { self.play(&self.ping); }

    fn play(&self, sound: &sdl2::mixer::Music) -> () {
        match sound.play(1) {
            Err(e) => println!("Error playing sound: {:?}", e),
            _ => ()
        };
    }
}
