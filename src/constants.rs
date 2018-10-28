use vector::{Vector};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    Black,
    White,
    Purple,
}

#[derive(Debug)]
pub struct Constants {
    pub window_width: i32,
    pub window_height: i32,
    pub paddle_width: i32,
    pub paddle_height: i32,
    pub paddle_segment: i32,
    pub max_paddle_speed: f64,
    pub ball_width: i32,
    pub ball_height: i32,
    pub max_ball_speed: Vector,
    pub ball_color: Color,
    pub background_color: Color,
    pub color_seqence: [Color; 3],
}

pub const OUTER_CONSTANTS: Constants = Constants {
    window_width: 1200,
    window_height: 800,
    paddle_width: 35,
    paddle_height: 150,
    paddle_segment: 25,
    max_paddle_speed: 0.15,
    ball_width: 510,
    ball_height: 310,
    max_ball_speed: Vector { x: 0.15, y: 0.8 },
    ball_color: Color::Purple,
    background_color: Color::Black,
    color_seqence: [Color::Red, Color::Green, Color::Blue],
};

pub const INNER_CONSTANTS: Constants = Constants {
    window_width: 500,
    window_height: 300,
    paddle_width: 20,
    paddle_height: 90,
    paddle_segment: 15,
    max_paddle_speed: 0.25,
    ball_width: 15,
    ball_height: 15,
    max_ball_speed: Vector { x: 0.45, y: 1.5 },
    ball_color: Color::Purple,
    background_color: Color::Black,
    color_seqence: [Color::Red, Color::Green, Color::Blue],
};
