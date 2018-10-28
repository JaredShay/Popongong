use rand::{thread_rng, Rng};

use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

use std::collections::HashMap;

use vector::Vector;

use component::{Paddle, Ball, Component};

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
    pub background_color: Color,
    pub paddle_one: Paddle,
    pub paddle_two: Paddle,
    pub ball: Ball,
    pub state: GameStates,
    pub color_index: usize,
    constants: Constants,
}

impl Game {
    pub fn new(constants: Constants) -> Game {
        let paddle_one = Paddle::new(
            Vector { x: 0.0, y: 0.0 },
            constants.paddle_width as u32,
            constants.paddle_height as u32,
            constants.max_paddle_speed,
            constants.color_seqence[0].clone(),
        );

        let paddle_two = Paddle::new(
            Vector {
                x: (constants.window_width - constants.paddle_width) as f64,
                y: 0.0
            },
            constants.paddle_width as u32,
            constants.paddle_height as u32,
            constants.max_paddle_speed,
            constants.color_seqence[0].clone(),
        );

        let ball_x = constants.window_width / 2 - constants.ball_width / 2;
        let ball_y = constants.window_height / 2 - constants.ball_height / 2;

        let ball_starting_pos = Vector {
                x: ball_x as f64,
                y: ball_y as f64,
            };

        let y_sign_vals = vec![-1.0, 1.0];
        let y_sign = thread_rng().choose(&y_sign_vals).unwrap();
        let starting_ball_y_vel = thread_rng().gen_range(0.1, constants.max_ball_speed.y * 0.75) * y_sign;

        let x_sign_vals = vec![-1.0, 1.0];
        let x_sign = thread_rng().choose(&x_sign_vals).unwrap();
        let starting_ball_x_vel = constants.max_ball_speed.x * x_sign;

        let ball = Ball::new(
            ball_starting_pos,
            constants.ball_width as u32,
            constants.ball_height as u32,
            Vector { x: starting_ball_x_vel, y: starting_ball_y_vel },
            constants.ball_color.clone()
        );

        Game {
            background: Rect::new(0, 0, constants.window_width as u32, constants.window_height as u32),
            background_color: constants.background_color.clone(),
            paddle_one: paddle_one,
            paddle_two: paddle_two,
            ball: ball,
            state: GameStates::Paused,
            color_index: 0,
            constants: constants,
        }
    }

    pub fn start(&mut self) -> () {
        self.state = GameStates::Playing;
    }

    pub fn components(&mut self, origin: &Vector) -> Vec<Component> {
        self.background.set_x(origin.x as i32);
        self.background.set_y(origin.y as i32);

        let mut components = vec![
            (&self.background, &self.background_color)
        ];

        components.append(&mut self.paddle_one.components(&origin));
        components.append(&mut self.paddle_two.components(&origin));
        components.append(&mut self.ball.components(&origin));

        return components;
    }

    pub fn play_pause(&mut self) -> () {
        if self.state == GameStates::Paused {
            self.state = GameStates::Playing;
        } else {
            self.state = GameStates::Paused;
        }
    }

    pub fn next_color(&mut self) -> () {
        if self.color_index < 2 {
            self.color_index = self.color_index + 1;
        } else {
            self.color_index = 0;
        }

        self.paddle_one.set_color(&self.constants.color_seqence[self.color_index]);
        self.paddle_two.set_color(&self.constants.color_seqence[self.color_index]);

        self.paddle_one.reset_hits();
        self.paddle_two.reset_hits();
    }

    pub fn hits (&self) -> u8 {
        return self.paddle_one.hits + self.paddle_two.hits;
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

            // Collisions
            // - ball_top_screen_edge
            // - ball_bottom_screen_edge
            // - ball_left_screen_edge
            // - ball_right_screen_edge
            // - ball_paddle_one
            // - ball_paddle_two

            self.ball.update(delta_ms);

            // Edge collisions
            if self.ball_collides_with_top() || self.ball_collides_with_bottom() {
                self.ball.flip_y();
            }

            if self.ball_collides_with_left() {
                self.ball.flip_x();
                self.paddle_one.miss();
            }

            if self.ball_collides_with_right() {
                self.ball.flip_x();
                self.paddle_two.miss();
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
                self.paddle_one.hit();
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
                self.paddle_two.hit();
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
        distance / self.constants.paddle_height as f64 / 2.0 * self.constants.max_ball_speed.y
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
