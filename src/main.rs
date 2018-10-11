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
use component::{Paddle, Ball};

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

const PADDLE_WIDTH: i32 = 40;
const PADDLE_HEIGHT: i32 = 240;
const PADDLE_SEGMENT: i32 = 40;

const BALL_WIDTH: i32 = 100;
const BALL_HEIGHT: i32 = 100;
const MAX_BALL_SPEED: f64 = 1.5;

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

    let window = video_subsystem.window("Not Pong", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        //.fullscreen()
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

    let mut paddle_one = Paddle::new(
                Vector{ x: 0.0, y: 0.0 },
                PADDLE_WIDTH as u32,
                PADDLE_HEIGHT as u32,
                0.5
            );

    let mut paddle_two = Paddle::new(
                Vector{ x: (WINDOW_WIDTH - PADDLE_WIDTH) as f64, y: 0.0 },
                PADDLE_WIDTH as u32,
                PADDLE_HEIGHT as u32,
                0.5
            );

    // calculate starting velocity
    // Pick random point on screen
    // Calc norm vector from ball
    // Multiple by some speed multiplier
    let target = Vector { x: 0.0, y: WINDOW_HEIGHT as f64 / 2.0 };

    let starting_pos = Vector {
            x: (WINDOW_WIDTH as f64) / 2.0 - (BALL_WIDTH as f64)  / 2.0,
            y: (WINDOW_HEIGHT as f64) / 2.0 - (BALL_HEIGHT as f64) / 2.0
        };

    // TODO: 0.4 is just magic to make the init velocity feel right
    let velocity = target.subtract(&starting_pos).normalize().product(0.4);

    let mut ball = Ball::new(
                starting_pos,
                BALL_WIDTH as u32,
                BALL_HEIGHT as u32,
                velocity
            );

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

    // Initial render
    canvas.clear();
    canvas.copy(&texture, None, *paddle_one.sdl_rect()).unwrap();
    canvas.copy(&texture, None, *paddle_two.sdl_rect()).unwrap();
    canvas.copy(&texture, None, *ball.sdl_rect()).unwrap();
    canvas.present();

    let mut delta_ms: u64;
    let mut prev_time = Instant::now();
    let mut curr_time;

    fn ball_collides_with_paddle_one(ball: &Ball, paddle_one: &Paddle) -> bool {
        ball.is_moving_left() &&
            ball.left_edge() <= paddle_one.right_edge() &&
            ball_within_paddle_collision_range(&ball, &paddle_one)
    }

    fn ball_collides_with_paddle_two(ball: &Ball, paddle_two: &Paddle) -> bool {
        ball.is_moving_right() &&
            ball.right_edge() >= WINDOW_WIDTH - paddle_two.width as i32 &&
            ball_within_paddle_collision_range(&ball, &paddle_two)
    }

    fn ball_within_paddle_collision_range(ball: &Ball, paddle: &Paddle) -> bool {
        ball.bottom_edge() >= paddle.top_edge() &&
            ball.top_edge() <= paddle.bottom_edge()
    }

    fn ball_collides_with_top(ball: &Ball) -> bool {
        ball.is_moving_up() && ball.top_edge() <= 0
    }

    fn ball_collides_with_bottom(ball: &Ball) -> bool {
        ball.is_moving_down() && ball.bottom_edge() >= WINDOW_HEIGHT
    }

    fn ball_collides_with_left(ball: &Ball) -> bool {
        ball.is_moving_left() && ball.left_edge() <= 0
    }

    fn ball_collides_with_right(ball: &Ball) -> bool {
        ball.is_moving_right() && ball.right_edge() >= WINDOW_WIDTH
    }

    // TODO: A nice enhancment here would be to factor in paddle velocity. If
    // the paddle is stationary don't apply any modification.
    fn vel_modifier(distance: f64, paddle: &Paddle) -> f64 {
        distance / paddle.height as f64 / 2.0 * MAX_BALL_SPEED
    }

    fn ball_collides_with_paddle_extremity(distance: f64) -> bool {
        distance as i32 > PADDLE_SEGMENT * 2
    }

    fn ball_moves_into_bottom_half(ball: &Ball, paddle: &Paddle) -> bool {
        ball.is_moving_up() && ball.top_edge() > paddle.center().y as i32
    }

    fn ball_moves_into_top_half(ball: &Ball, paddle: &Paddle) -> bool {
        ball.is_moving_down() && ball.bottom_edge() < paddle.center().y as i32
    }

    fn handle_ball_paddle_collision(ball: &mut Ball, paddle: &Paddle) {
        let collision_distance = ball.distance_to(&paddle).y.abs();

        if ball_collides_with_paddle_extremity(collision_distance) &&
            ball_moves_into_bottom_half(&ball, &paddle) ||
            ball_moves_into_top_half(&ball, &paddle) {

            ball.flip_y();
        }

        ball.set_velocity_y_magnitude(
            vel_modifier(collision_distance, &paddle)
        );

        ball.flip_x();
    }

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

        for (key, _) in &keys_pressed {
            match key {
                Keycode::Up => { paddle_two.up(delta_ms, 0.0); },
                Keycode::Down => { paddle_two.down(delta_ms, WINDOW_HEIGHT as f64); },
                Keycode::W => { paddle_one.up(delta_ms, 0.0); },
                Keycode::S => { paddle_one.down(delta_ms, WINDOW_HEIGHT as f64); },
                _ => {}
            }
        }

        ball.update(delta_ms);

        // Edge collisions
        if ball_collides_with_top(&ball) || ball_collides_with_bottom(&ball) {
            ball.flip_y();
        }

        if ball_collides_with_left(&ball) || ball_collides_with_right(&ball) {
            ball.flip_x();
        }

        //  Paddle one collision
        if ball_collides_with_paddle_one(&ball, &paddle_one) {
            handle_ball_paddle_collision(&mut ball, &paddle_one);
        }

        if ball_collides_with_paddle_two(&ball, &paddle_two) {
            handle_ball_paddle_collision(&mut ball, &paddle_two);
        }

        // render
        //
        // This should be implemented by looping over all game elements that
        // can be copied to the canvas, performing a copy if they are updated,
        // then calling render
        canvas.clear();
        // Texture, source, destination.
        //
        // Passing source of None means the entire texture is copied
        canvas.copy(&texture, None, *paddle_one.sdl_rect()).unwrap();
        canvas.copy(&texture, None, *paddle_two.sdl_rect()).unwrap();
        canvas.copy(&texture, None, *ball.sdl_rect()).unwrap();
        canvas.present();

        // Update time. Conceptually easier for me to see this here
        prev_time = curr_time;
    }
}
