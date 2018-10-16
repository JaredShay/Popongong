extern crate sdl2;

use std::collections::HashMap;
use vector::{Vector};
use constants::{Color};
use game::{Game};

pub fn render(
    outer_game: &mut Game,
    inner_game: &mut Game,
    textures: &HashMap<Color, sdl2::render::Texture<'_>>,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>
) -> () {
    canvas.clear();

    let outer_origin = Vector { x: 0.0, y: 0.0 };

    for component in outer_game.components(&outer_origin).iter_mut() {
        canvas.copy(
            &textures.get(&component.1).unwrap(),
            None,
            *component.0
        ).unwrap();
    }

    let inner_origin = Vector {
        x: outer_game.ball.pos.x + 5.0,
        y: outer_game.ball.pos.y + 5.0,
    };

    for component in inner_game.components(&inner_origin).iter_mut() {
        canvas.copy(
            &textures.get(&component.1).unwrap(),
            None,
            *component.0
        ).unwrap();
    }

    canvas.present();
}
