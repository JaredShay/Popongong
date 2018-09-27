extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::pixels::PixelFormatEnum;

// This is heavily commented and the comments may lie. It is my best effort to
// document things I don't understand very well.
//
// TODO:
// - unwrap is discouraged as it will panic if None is returned. Use pattern
//   matching instead of unwrap to handle that case explicitly.

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const PADDLE_WIDTH: u32 = 128;
const PADDLE_HEIGHT: u32 = 256;

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

    let window = video_subsystem.window("Not Pong", WINDOW_WIDTH, WINDOW_HEIGHT)
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
    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24, PADDLE_WIDTH, PADDLE_HEIGHT).unwrap();

    // lock texture to perform direct pixel writes. If this recalculation is
    // heavy consider caching where possible or moving the logic to a shader.
    // Using a shader will allow the graphics card to do the heavy lifting but
    // will be harder to make portable.
    //
    // This is a one time operation outside of the main loop so this is fine.
    //
    // The second argument to the supplied closure here is `pitch` which I
    // believe is "the length of a row of pixels in bytes".
    let texture_buffer = [0; PADDLE_WIDTH as usize * PADDLE_HEIGHT as usize * 3]; // * 3 for RGB
    texture.with_lock(None, |buffer: &mut [u8], _| {
        buffer.clone_from_slice(&texture_buffer);
    }).unwrap();

    // Initial render

    // Set draw color to white
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    // Always a good idea to clear before rendering
    canvas.clear();

    // render paddle 1 to the top left corner
    canvas.copy(&texture, None, Some(Rect::new(0, 0, PADDLE_WIDTH, PADDLE_HEIGHT))).unwrap();
    // render paddle 2 to the top right corner
    canvas.copy(&texture, None, Some(Rect::new((WINDOW_WIDTH - PADDLE_WIDTH) as i32, 0, PADDLE_WIDTH, PADDLE_HEIGHT))).unwrap();

    canvas.present();

    // Get a reference to the SDL "event pump".
    //
    // Only one event pump may exist for a given program. Pretty sure this is
    // just a event queue of some kind. Should be initialized in the thread
    // that initialized the video subsystem, and both should be initialized in
    // the main thread.
    let mut event_pump = sdl_context.event_pump().unwrap();

    'main: loop {
        // Grab lastest events and iterate over them
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    println!("Right");
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    println!("Down");
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    println!("Left");
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    println!("Up");
                },
                _ => {}
            }
        }
    }
    // Sleep a bit I guess... Just cargo culted this in.
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}
