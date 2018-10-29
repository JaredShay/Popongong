use sdl2;

pub struct Sounds<'a> {
    pub ping: &'a sdl2::mixer::Music<'a>,
    pub pong: &'a sdl2::mixer::Music<'a>
}

impl<'a> Sounds<'a> {
    pub fn ping(&self) -> () {
        self.ping.play(1);
    }

    pub fn pong(&self) -> () {
        self.pong.play(1);
    }
}
