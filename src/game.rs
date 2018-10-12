extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use std::collections::HashMap;

use vector::Vector;

use component::{Paddle, Ball};

use constants::{
    WINDOW_WIDTH,
    WINDOW_HEIGHT,
    PADDLE_WIDTH,
    PADDLE_HEIGHT,
    PADDLE_SEGMENT,
    BALL_WIDTH,
    BALL_HEIGHT,
    MAX_BALL_SPEED,
};

#[derive(Debug, PartialEq)]
pub enum GameStates {
    Playing,
//    Finished,
    Paused
}

#[derive(Debug)]
pub struct Game {
    pub background: Rect,
    pub paddle_one: Paddle,
    pub paddle_two: Paddle,
    pub ball: Ball,
    pub state: GameStates,
    scale: i32
}

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

fn ball_collides_with_bottom(ball: &Ball, scale: i32) -> bool {
    ball.is_moving_down() && ball.bottom_edge() >= WINDOW_HEIGHT / scale
}

fn ball_collides_with_left(ball: &Ball) -> bool {
    ball.is_moving_left() && ball.left_edge() <= 0
}

fn ball_collides_with_right(ball: &Ball, scale: i32) -> bool {
    ball.is_moving_right() && ball.right_edge() >= WINDOW_WIDTH / scale
}

// TODO: A nice enhancment here would be to factor in paddle velocity. If
// the paddle is stationary don't apply any modification.
fn vel_modifier(distance: f64, paddle: &Paddle) -> f64 {
    distance / paddle.height as f64 / 2.0 * MAX_BALL_SPEED
}

fn ball_collides_with_paddle_extremity(distance: f64, scale: i32) -> bool {
    distance as i32 > (PADDLE_SEGMENT * 2) / scale
}

fn ball_moves_into_bottom_half(ball: &Ball, paddle: &Paddle) -> bool {
    ball.is_moving_up() && ball.top_edge() > paddle.center().y as i32
}

fn ball_moves_into_top_half(ball: &Ball, paddle: &Paddle) -> bool {
    ball.is_moving_down() && ball.bottom_edge() < paddle.center().y as i32
}

fn handle_ball_paddle_collision(ball: &mut Ball, paddle: &Paddle, scale: i32) {
    let collision_distance = ball.distance_to(&paddle).y.abs();

    if ball_collides_with_paddle_extremity(collision_distance, scale) &&
        ball_moves_into_bottom_half(&ball, &paddle) ||
        ball_moves_into_top_half(&ball, &paddle) {

        ball.flip_y();
    }

    ball.set_velocity_y_magnitude(
        vel_modifier(collision_distance, &paddle)
    );

    ball.flip_x();
}

impl Game {
    pub fn new(scale: i32) -> Game {
        let paddle_one = Paddle::new(
            Vector { x: 0.0, y: 0.0 },
            (PADDLE_WIDTH / scale) as u32,
            (PADDLE_HEIGHT / scale) as u32,
            0.5
        );

        let paddle_two = Paddle::new(
            Vector {
                x: (WINDOW_WIDTH / scale - PADDLE_WIDTH / scale) as f64,
                y: 0.0
            },
            (PADDLE_WIDTH / scale) as u32,
            (PADDLE_HEIGHT / scale) as u32,
            0.5
        );

        // calculate starting velocity
        // Pick random point on screen
        // Calc norm vector from ball
        // Multiple by some speed multiplier
        let target = Vector { x: 0.0, y: (WINDOW_WIDTH / scale) as f64 / 2.0 };

        let ball_starting_pos = Vector {
                x: ((WINDOW_WIDTH / scale) as f64) / 2.0 - ((BALL_WIDTH / scale) as f64)  / 2.0,
                y: ((WINDOW_HEIGHT / scale) as f64) / 2.0 -( (BALL_HEIGHT / scale) as f64) / 2.0
            };

        // TODO: 0.4 is just magic to make the init velocity feel right.
        let ball_velocity = target.subtract(&ball_starting_pos).normalize().product(0.4);

        let ball = Ball::new(
            ball_starting_pos,
            (BALL_WIDTH / scale) as u32,
            (BALL_HEIGHT / scale) as u32,
            ball_velocity
        );

        Game {
            background: Rect::new(
                0,
                0,
                WINDOW_WIDTH as u32 / scale as u32,
                WINDOW_HEIGHT as u32 / scale as u32
            ),
            paddle_one: paddle_one,
            paddle_two: paddle_two,
            ball: ball,
            state: GameStates::Paused,
            scale: scale
        }
    }

    pub fn start(&mut self) -> () {
        self.state = GameStates::Playing;
    }

    pub fn background(&mut self, origin: &Vector) -> &sdl2::rect::Rect {
        self.background.set_x(origin.x as i32);
        self.background.set_y(origin.y as i32);

        return &self.background;
    }

    pub fn update(
        &mut self,
        keys_pressed: &HashMap<&Keycode, bool>,
        delta_ms: u64
    ) -> () {
        if self.state != GameStates::Paused {
            for (key, _) in keys_pressed {
                match key {
                    Keycode::Up => {
                        self.paddle_two.up(delta_ms, 0.0);
                    },
                    Keycode::Down => {
                        self.paddle_two.down(
                            delta_ms,
                            (WINDOW_HEIGHT / self.scale) as f64
                        );
                    },
                    Keycode::W => {
                        self.paddle_one.up(delta_ms, 0.0);
                    },
                    Keycode::S => {
                        self.paddle_one.down(
                            delta_ms,
                            (WINDOW_HEIGHT / self.scale) as f64
                        );
                    },
                    _ => {}
                }
            }

            self.ball.update(delta_ms);

            // Edge collisions
            if ball_collides_with_top(&self.ball) ||
                ball_collides_with_bottom(&self.ball, self.scale) {
                self.ball.flip_y();
            }

            if ball_collides_with_left(&self.ball) ||
                ball_collides_with_right(&self.ball, self.scale) {
                self.ball.flip_x();
            }

            //  Paddle one collision
            if ball_collides_with_paddle_one(&self.ball, &self.paddle_one) {
                handle_ball_paddle_collision(&mut self.ball, &self.paddle_one, self.scale);
            }

            if ball_collides_with_paddle_two(&self.ball, &self.paddle_two) {
                handle_ball_paddle_collision(&mut self.ball, &self.paddle_two, self.scale);
            }
        }
    }
}
