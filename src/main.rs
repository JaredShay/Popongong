extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

// This is heavily commented and the comments may lie. It is my best effort to
// document things I don't understand very well.
//
// TODO:
// - unwrap is discouraged as it will panic if None is returned. Use pattern
//   matching instead of unwrap to handle that case explicitly.
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

    let window = video_subsystem.window("Not Pong", 800, 600)
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

    // This appears to fill the whole canvas
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
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
                _ => {}
            }
        }
    }
    // Sleep a bit I guess... Just cargo culted this in.
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}
