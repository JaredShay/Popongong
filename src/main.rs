extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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

const BACKGROUND_COLOR: (u8, u8, u8) = (255, 255, 255); // white

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

    // This looks like garbage. Tuple splatting doesn't exist. Fun to say though
    canvas.set_draw_color(Color::RGB(BACKGROUND_COLOR.0, BACKGROUND_COLOR.1, BACKGROUND_COLOR.2));

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
        // could just loop and initialize each el here. This is just because I
        // was curious how to copy an existing buffer in rust. This only works
        // because I'm setting each el to 0 which makes each pixel RGB(0, 0, 0).
        // Would need to do something else to get a color that had different
        // values.
        buffer.clone_from_slice(&texture_buffer);
    }).unwrap();

    // Rect has a set_<x|y> method which is all we need to move them about
    // Starting co-ords are top left.
    let mut paddle_one = Rect::new(0, 0, PADDLE_WIDTH, PADDLE_HEIGHT);
    // Top right for this one
    let mut paddle_two = Rect::new((WINDOW_WIDTH - PADDLE_WIDTH) as i32, 0, PADDLE_WIDTH, PADDLE_HEIGHT);

    // Initial render
    // Always a good idea to clear before rendering
    canvas.clear();
    canvas.copy(&texture, None, Some(paddle_one)).unwrap();
    canvas.copy(&texture, None, Some(paddle_two)).unwrap();
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
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    // this doesn't feel very idiomatic. Referencing the value
                    // inside the set all throws a compiler error. This just
                    // breaks the statement into two lines to avoid any
                    // confusion

                    let y = paddle_one.y;
                    paddle_one.set_y(y + 10);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    // see above comment
                    let y = paddle_one.y;
                    paddle_one.set_y(y - 10);
                },
                _ => {}
            }
        }

        canvas.clear();
        canvas.copy(&texture, None, Some(paddle_one)).unwrap();
        canvas.copy(&texture, None, Some(paddle_two)).unwrap();
        canvas.present();
    }
}
