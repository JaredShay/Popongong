extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::PixelFormatEnum;

use std::collections::HashMap;

// This is heavily commented and the comments may lie. It is my best effort to
// document things I don't understand very well.
//
// TODO:
// - unwrap is discouraged as it will panic if None is returned. Use pattern
//   matching instead of unwrap to handle that case explicitly.

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const PADDLE_WIDTH: u32 = 100;
const PADDLE_HEIGHT: u32 = 250;

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
        buffer.copy_from_slice(&texture_buffer);
    }).unwrap();

    struct Paddle {
        x: i32,
        y: i32,
        rect: Rect
    }

    impl Paddle {
        const MOVEMENT_STEP: i32 = 10;

        fn new(x: i32, y: i32) -> Paddle {
            return Paddle {
                x: x,
                y: y,
                rect: Rect::new(x, y, PADDLE_WIDTH, PADDLE_HEIGHT)
            };
        }

        fn sdl_rect(&mut self) -> &sdl2::rect::Rect {
            self.rect.set_x(self.x);
            self.rect.set_y(self.y);

            return &self.rect;
        }

        fn up(&mut self) -> () {
            if (self.y - Self::MOVEMENT_STEP) >= 0 {
                self.y = self.y - Self::MOVEMENT_STEP;
            }
        }

        fn down(&mut self) -> () {
            // TODO: this check won't pass if the remaining space is < step size.
            if (self.y + PADDLE_HEIGHT as i32 + Self::MOVEMENT_STEP) <= WINDOW_HEIGHT as i32 {
                self.y = self.y + Self::MOVEMENT_STEP;
            }
        }
    }

    let mut paddle_one = Paddle::new(0, 0);
    let mut paddle_two = Paddle::new((WINDOW_WIDTH - PADDLE_WIDTH) as i32, 0);

    // Initial render
    // Always a good idea to clear before rendering
    canvas.clear();
    canvas.copy(&texture, None, *paddle_one.sdl_rect()).unwrap();
    canvas.copy(&texture, None, *paddle_two.sdl_rect()).unwrap();
    canvas.present();

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

    'main: loop {
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

        for (key, _) in &keys_pressed {
            match key {
                Keycode::Up => { paddle_two.up(); },
                Keycode::Down => { paddle_two.down(); },
                Keycode::W => { paddle_one.up(); },
                Keycode::S => { paddle_one.down(); },
                _ => {}
            }
        }

        canvas.clear();
        // Texture, source, destination.
        //
        // Passing source of None means the entire texture is copied
        canvas.copy(&texture, None, *paddle_one.sdl_rect()).unwrap();
        canvas.copy(&texture, None, *paddle_two.sdl_rect()).unwrap();
        canvas.present();
    }
}
