extern crate sdl2;

use std::collections::HashMap;
use vector::{Vector};
use component::{Components};
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

    for component in outer_game.components().iter_mut() {
        match component {
            Components::Paddle(paddle) => {
                canvas.copy(
                    &textures.get(&paddle.color).unwrap(),
                    None,
                    *paddle.sdl_rect(&outer_origin)
                ).unwrap();
            },
            Components::Ball(ball) => {
                canvas.copy(
                    &textures.get(&ball.color).unwrap(),
                    None,
                    *ball.sdl_rect(&outer_origin)
                ).unwrap();
            },
            Components::Background(background) => {
                canvas.copy(
                    &textures.get(&background.color).unwrap(),
                    None,
                    *background.sdl_rect(&outer_origin)
                ).unwrap();
            },
        }
    }

    let inner_origin = Vector {
        x: outer_game.ball.pos.x + 5.0,
        y: outer_game.ball.pos.y + 5.0,
    };

    for component in inner_game.components().iter_mut() {
        match component {
            Components::Paddle(paddle) => {
                canvas.copy(
                    &textures.get(&paddle.color).unwrap(),
                    None,
                    *paddle.sdl_rect(&inner_origin)
                ).unwrap();
            },
            Components::Ball(ball) => {
                canvas.copy(
                    &textures.get(&ball.color).unwrap(),
                    None,
                    *ball.sdl_rect(&inner_origin)
                ).unwrap();
            },
            Components::Background(background) => {
                canvas.copy(
                    &textures.get(&background.color).unwrap(),
                    None,
                    *background.sdl_rect(&inner_origin)
                ).unwrap();
            },
        }
    }

    canvas.present();
}
