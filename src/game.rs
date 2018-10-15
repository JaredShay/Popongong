extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use std::collections::HashMap;

use vector::Vector;

use component::{Paddle, Ball};

use constants::{Color, Constants};

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
    constants: Constants,
    scale: i32
}

impl Game {
    pub fn new(scale: i32, constants: Constants) -> Game {
        let paddle_one = Paddle::new(
            Vector { x: 0.0, y: 0.0 },
            constants.paddle_width as u32,
            constants.paddle_height as u32,
            0.5,
            Color::Black,
        );

        let paddle_two = Paddle::new(
            Vector {
                x: (constants.window_width - constants.paddle_width) as f64,
                y: 0.0
            },
            constants.paddle_width as u32,
            constants.paddle_height as u32,
            0.5,
            Color::Black,
        );

        // calculate starting velocity
        // Pick random point on screen
        // Calc norm vector from ball
        // Multiple by some speed multiplier
        let target = Vector { x: 0.0, y: constants.window_width as f64 / 2.0 };

        let ball_x = constants.window_width / 2 - constants.ball_width / 2;
        let ball_y = constants.window_height / 2 - constants.ball_height / 2;

        let ball_starting_pos = Vector {
                x: ball_x as f64,
                y: ball_y as f64,
            };

        // TODO: 0.4 is just magic to make the init velocity feel right.
        let ball_velocity = target
            .subtract(&ball_starting_pos)
            .normalize()
            .product(0.4);

        let ball = Ball::new(
            ball_starting_pos,
            constants.ball_width as u32,
            constants.ball_height as u32,
            ball_velocity,
            Color::Black
        );

        Game {
            background: Rect::new(
                0,
                0,
                constants.window_width as u32,
                constants.window_height as u32,
            ),
            paddle_one: paddle_one,
            paddle_two: paddle_two,
            ball: ball,
            state: GameStates::Paused,
            scale: scale,
            constants: constants,
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
                            (self.constants.window_height) as f64
                        );
                    },
                    Keycode::W => {
                        self.paddle_one.up(delta_ms, 0.0);
                    },
                    Keycode::S => {
                        self.paddle_one.down(
                            delta_ms,
                            self.constants.window_height as f64
                        );
                    },
                    _ => {}
                }
            }

            self.ball.update(delta_ms);

            // Edge collisions
            if self.ball_collides_with_top() || self.ball_collides_with_bottom() {
                self.ball.flip_y();
            }

            if self.ball_collides_with_left() || self.ball_collides_with_right() {
                self.ball.flip_x();
            }

            if self.ball_collides_with_paddle_one() {
                let collision_distance = self.ball.distance_to(&self.paddle_one).y.abs();
                let new_velocity = self.vel_modifier(collision_distance);

                if self.ball_collides_with_paddle_extremity(collision_distance) &&
                    self.ball_moves_into_bottom_half(&self.paddle_one) ||
                    self.ball_moves_into_top_half(&self.paddle_one) {

                    self.ball.flip_y();
                }

                self.ball.set_velocity_y_magnitude(new_velocity);
                self.ball.flip_x();
            }

            if self.ball_collides_with_paddle_two() {
                let collision_distance = self.ball.distance_to(&self.paddle_two).y.abs();
                let new_velocity = self.vel_modifier(collision_distance);

                if self.ball_collides_with_paddle_extremity(collision_distance) &&
                    self.ball_moves_into_bottom_half(&self.paddle_two) ||
                    self.ball_moves_into_top_half(&self.paddle_two) {

                    self.ball.flip_y();
                }

                self.ball.set_velocity_y_magnitude(new_velocity);
                self.ball.flip_x();
            }
        }
    }

    fn ball_collides_with_paddle_one(&self) -> bool {
        self.ball.is_moving_left() &&
            self.ball.left_edge() <= self.paddle_one.right_edge() &&
            self.ball_within_paddle_collision_range(&self.paddle_one)
    }

    fn ball_collides_with_paddle_two(&self) -> bool {
        self.ball.is_moving_right() &&
            self.ball.right_edge() >= self.constants.window_width - self.paddle_two.width as i32 &&
            self.ball_within_paddle_collision_range(&self.paddle_two)
    }

    fn ball_within_paddle_collision_range(&self, paddle: &Paddle) -> bool {
        self.ball.bottom_edge() >= paddle.top_edge() &&
            self.ball.top_edge() <= paddle.bottom_edge()
    }

    fn ball_collides_with_top(&self) -> bool {
        self.ball.is_moving_up() && self.ball.top_edge() <= 0
    }

    fn ball_collides_with_bottom(&self) -> bool {
        self.ball.is_moving_down() && self.ball.bottom_edge() >= self.constants.window_height
    }

    fn ball_collides_with_left(&self) -> bool {
        self.ball.is_moving_left() && self.ball.left_edge() <= 0
    }

    fn ball_collides_with_right(&self) -> bool {
        self.ball.is_moving_right() && self.ball.right_edge() >= self.constants.window_width
    }

    // TODO: A nice enhancment here would be to factor in paddle velocity. If
    // the paddle is stationary don't apply any modification.
    fn vel_modifier(&self, distance: f64) -> f64 {
        distance / self.constants.paddle_height as f64 / 2.0 * self.constants.max_ball_speed
    }

    fn ball_collides_with_paddle_extremity(&self, distance: f64) -> bool {
        // Edge 2/6th of the paddle
        distance as i32 > (self.constants.paddle_segment * 2)
    }

    fn ball_moves_into_bottom_half(&self, paddle: &Paddle) -> bool {
        self.ball.is_moving_up() && self.ball.top_edge() > paddle.center().y as i32
    }

    fn ball_moves_into_top_half(&self, paddle: &Paddle) -> bool {
        self.ball.is_moving_down() && self.ball.bottom_edge() < paddle.center().y as i32
    }
}
