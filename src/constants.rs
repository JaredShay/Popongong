pub const BACKGROUND_COLOR: (u8, u8, u8) = (255, 255, 255); // white

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
}

pub const OUTER_CONSTANTS: Constants = Constants {
    window_width: 1200,
    window_height: 800,
    paddle_width: 90,
    paddle_height: 360,
    paddle_segment: 60,
    max_paddle_speed: 0.5,
    ball_width: 600,
    ball_height: 400,
    max_ball_speed: 1.5,
};

pub const INNER_CONSTANTS: Constants = Constants {
    window_width: OUTER_CONSTANTS.ball_width,
    window_height: OUTER_CONSTANTS.ball_height,
    paddle_width: 30,
    paddle_height: 120,
    paddle_segment: 15,
    max_paddle_speed: 0.5,
    ball_width: 75,
    ball_height: 75,
    max_ball_speed: 1.0,
};
