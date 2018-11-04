use sdl2;

use std::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Sounds<'a> {
    sounds: HashMap<String, sdl2::mixer::Music<'a>>
}

impl<'a> Sounds<'a> {
    pub fn new() -> Sounds<'a> {
        let mut sounds: HashMap<String, sdl2::mixer::Music> = HashMap::new();

        for level in 1..4 {
            for sound in 1..6 {
                sounds.insert(
                    format!("paddle_{}_{}", level, sound).to_string(),
                    sdl2::mixer::Music::from_file(
                        Path::new(&format!("./sounds/paddle_{}_{}.wav", level, sound))
                    ).unwrap()
                );
            }
        }

        Sounds { sounds: sounds }
    }

    pub fn play(&self, sound: String) -> () {
        // 1 is the number of loops
        match self.sounds.get(&sound).unwrap().play(1) {
            Err(e) => println!("Error playing sound: {:?}", e),
            _ => ()
        };
    }
}
