extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;

use std::collections::HashMap;

use constants;

pub fn init_textures<'a>(
    creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>
) -> HashMap<constants::Color, sdl2::render::Texture<'a>> {
    let black = create_texture(&creator, 0, 13, 40);
    let white = create_texture(&creator, 255, 255, 255);
    let red = create_texture(&creator, 221, 83, 134);
    let green = create_texture(&creator, 70, 184, 135);
    let blue = create_texture(&creator, 7, 137, 199);
    let purple = create_texture(&creator, 253, 184, 119); // not purple

    let mut textures = HashMap::new();

    textures.insert(constants::Color::Black, black);
    textures.insert(constants::Color::White, white);
    textures.insert(constants::Color::Red, red);
    textures.insert(constants::Color::Green, green);
    textures.insert(constants::Color::Blue, blue);
    textures.insert(constants::Color::Purple, purple);

    return textures;
}

fn create_texture(
    creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    r: u8,
    g: u8,
    b: u8
) -> sdl2::render::Texture {
    let mut texture = creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 1, 1).unwrap();

    texture.with_lock(None, |buffer: &mut [u8], _| {
        buffer[0] = r;
        buffer[1] = g;
        buffer[2] = b;
    }).unwrap();

    return texture;
}
