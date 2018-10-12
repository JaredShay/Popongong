extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

use std::collections::HashMap;
use std::time::{Instant};

mod vector;
use vector::Vector;

mod component;

mod game;
use game::{Game};

mod constants;
use constants::{
    WINDOW_WIDTH,
    WINDOW_HEIGHT,
    PADDLE_WIDTH,
    PADDLE_HEIGHT,
    BACKGROUND_COLOR
};

fn main() {
    // Create an OpenGl context.
    //
    // A context stores all the state associated with an instance of OpenGl.
    let sdl_context = sdl2::init().unwrap();

    // Create a SDL Video Subsystem.
    //
    // SDL is comprised of 8 subsystems. Video is one of them and needs to be
    // manually initialsed.
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(
        "Popongong",
        WINDOW_WIDTH as u32,
        WINDOW_HEIGHT as u32
    )
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // Canvas manages and owns a Window (or Surface).
    //
    // Drawing into a canvas draws into a buffer until `present()` is called.
    // Generally a good idea to call `clear()` first.
    let mut canvas = window
        .into_canvas()
        .present_vsync() // don't render faster than screen refresh rate.
        .build()
        .unwrap();

    // This looks like garbage. Tuple splatting doesn't exist. Fun to say though
    canvas.set_draw_color(
        Color::RGB(
            BACKGROUND_COLOR.0, BACKGROUND_COLOR.1, BACKGROUND_COLOR.2
        )
    );

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24, PADDLE_WIDTH as u32, PADDLE_HEIGHT as u32).unwrap();

    // lock texture to perform direct pixel writes. If this recalculation is
    // heavy consider caching where possible or moving the logic to a shader.
    // Using a shader will allow the graphics card to do the heavy lifting but
    // will be harder to make portable.
    //
    // This is a one time operation outside of the main loop so this is fine.
    //
    // The second argument to the supplied closure here is `pitch` which I
    // believe is "the length of a row of pixels in bytes".
    texture.with_lock(None, |buffer: &mut [u8], _| {
        for el in 0..((PADDLE_WIDTH * PADDLE_HEIGHT * 3) as usize) {
            buffer[el] = 0;
        }
    }).unwrap();

    let mut red = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24, 1, 1).unwrap();

    red.with_lock(None, |buffer: &mut [u8], _| {
        buffer[0] = 255;
        buffer[1] = 0;
        buffer[2] = 0;
    }).unwrap();

    let mut green = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24, 1, 1).unwrap();

    green.with_lock(None, |buffer: &mut [u8], _| {
        buffer[0] = 0;
        buffer[1] = 255;
        buffer[2] = 0;
    }).unwrap();

    let mut outer_game = Game::new(1);
    let mut inner_game = Game::new(4);

    // Get a reference to the SDL "event pump".
    //
    // Only one event pump may exist for a given program. Pretty sure this is
    // just a event queue of some kind. Should be initialized in the thread
    // that initialized the video subsystem, and both should be initialized in
    // the main thread.
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Keep a hash map of key states. A KeyUp or KeyDown event will change the
    // state of the hash map and not directly modify a game element.
    let mut keys_pressed = HashMap::new();

    fn handle_key_press_events(event: sdl2::event::Event, keys_pressed: &mut HashMap<&Keycode, bool>) -> () {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                keys_pressed.insert(&Keycode::Up, true);
            },
            Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                keys_pressed.remove(&Keycode::Up);
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                keys_pressed.insert(&Keycode::Down, true);
            },
            Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                keys_pressed.remove(&Keycode::Down);
            },
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                keys_pressed.insert(&Keycode::W, true);
            },
            Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                keys_pressed.remove(&Keycode::W);
            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                keys_pressed.insert(&Keycode::S, true);
            },
            Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                keys_pressed.remove(&Keycode::S);
            },
            _ => {}
        }
    }

    fn to_ms(duration: std::time::Duration) -> u64 {
        duration.as_secs() * 1000 + duration.subsec_millis() as u64
    }

    let outer_origin = Vector { x: 0.0, y: 0.0 };

    // Initial render
    canvas.clear();
    canvas.copy(&red, None, *outer_game.background(&outer_origin)).unwrap();
    canvas.copy(&texture, None, *outer_game.paddle_one.sdl_rect(&outer_origin)).unwrap();
    canvas.copy(&texture, None, *outer_game.paddle_two.sdl_rect(&outer_origin)).unwrap();

    canvas.copy(&green, None, *inner_game.background(&outer_game.ball.pos)).unwrap();
    canvas.copy(&texture, None, *inner_game.paddle_one.sdl_rect(&outer_game.ball.pos)).unwrap();
    canvas.copy(&texture, None, *inner_game.paddle_two.sdl_rect(&outer_game.ball.pos)).unwrap();

    canvas.copy(&texture, None, *inner_game.ball.sdl_rect(&outer_game.ball.pos)).unwrap();
    canvas.present();

    let mut delta_ms: u64;
    let mut prev_time = Instant::now();
    let mut curr_time;

    outer_game.start();
    inner_game.start();

    'main: loop {
        curr_time = Instant::now();
        delta_ms = to_ms(curr_time.duration_since(prev_time));

        // Grab lastest events and iterate over them
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                },
                Event::KeyDown { .. } | Event::KeyUp { .. } => {
                    handle_key_press_events(event, &mut keys_pressed);
                }
                _ => {}
            }
        }

        outer_game.update(&keys_pressed, delta_ms);
        inner_game.update(&keys_pressed, delta_ms);
        //for component in &outer_game.components {
        //    //render components with 0,0 origin
        //}

        // render
        //
        // This should be implemented by looping over all game elements that
        // can be copied to the canvas, performing a copy if they are updated,
        // then calling render
        canvas.clear();
        // Texture, source, destination.
        //
        // Passing source of None means the entire texture is copied
        canvas.copy(&red, None, *outer_game.background(&outer_origin)).unwrap();
        canvas.copy(&texture, None, *outer_game.paddle_one.sdl_rect(&outer_origin)).unwrap();
        canvas.copy(&texture, None, *outer_game.paddle_two.sdl_rect(&outer_origin)).unwrap();

        canvas.copy(&green, None, *inner_game.background(&outer_game.ball.pos)).unwrap();
        canvas.copy(&texture, None, *inner_game.paddle_one.sdl_rect(&outer_game.ball.pos)).unwrap();
        canvas.copy(&texture, None, *inner_game.paddle_two.sdl_rect(&outer_game.ball.pos)).unwrap();

        canvas.copy(&texture, None, *inner_game.ball.sdl_rect(&outer_game.ball.pos)).unwrap();
        canvas.present();

        // Update time. Conceptually easier for me to see this here
        prev_time = curr_time;
    }
}
