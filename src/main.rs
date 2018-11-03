use std::collections::HashMap;
use std::time::{Instant};

extern crate sdl2;
extern crate rand;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod vector;
mod component;
mod game;
mod constants;
mod render;
mod textures;
mod sounds;

use constants::{OUTER_CONSTANTS, INNER_CONSTANTS};
use game::{Game};
use render::render;
use textures::init_textures;
use sounds::{Sounds};

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();
    let _audio_subsystem = sdl_context.audio().unwrap();

    let window = video_subsystem.window(
        "Popongong",
        OUTER_CONSTANTS.window_width as u32,
        OUTER_CONSTANTS.window_height as u32
    )
        .position_centered()
        .fullscreen()
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

    let texture_creator = canvas.texture_creator();
    let textures = init_textures(&texture_creator);

    let sounds = Sounds::new();

    let mut outer_game = Game::new(OUTER_CONSTANTS);
    let mut inner_game = Game::new(INNER_CONSTANTS);

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

    render(&mut outer_game, &mut inner_game, &textures, &mut canvas);

    let mut delta_ms: u64;
    let mut prev_time = Instant::now();
    let mut curr_time;

    outer_game.start();
    inner_game.start();

    'main: loop {
        curr_time = Instant::now();
        delta_ms = to_ms(curr_time.duration_since(prev_time));
        prev_time = curr_time;

        // Grab lastest events and iterate over them
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    outer_game.play_pause();
                    inner_game.play_pause();
                },
                Event::KeyDown { .. } | Event::KeyUp { .. } => {
                    handle_key_press_events(event, &mut keys_pressed);
                }
                _ => {}
            }
        }

        outer_game.update(&keys_pressed, delta_ms);
        inner_game.update(&keys_pressed, delta_ms);

        if outer_game.hits() == 10 && inner_game.hits() == 10 {
            outer_game.next_color();
            inner_game.next_color();
        }

        render(&mut outer_game, &mut inner_game, &textures, &mut canvas);
    }
}
