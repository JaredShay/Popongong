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
    pub max_ball_speed: f64,
    pub ball_color: Color,
    pub color_seqence: [Color; 3],
}

pub const OUTER_CONSTANTS: Constants = Constants {
    window_width: 1200,
    window_height: 800,
    paddle_width: 90,
    paddle_height: 360,
    paddle_segment: 60,
    max_paddle_speed: 0.5,
    ball_width: 610,
    ball_height: 410,
    max_ball_speed: 1.5,
    ball_color: Color::Purple,
    color_seqence: [Color::Red, Color::Green, Color::Blue],
};

pub const INNER_CONSTANTS: Constants = Constants {
    window_width: 600,
    window_height: 400,
    paddle_width: 30,
    paddle_height: 120,
    paddle_segment: 15,
    max_paddle_speed: 0.5,
    ball_width: 25,
    ball_height: 25,
    max_ball_speed: 1.0,
    ball_color: Color::Black,
    color_seqence: [Color::Red, Color::Green, Color::Blue],
};
