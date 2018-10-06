extern crate sdl2;
use sdl2::rect::Rect;

use point::Point;

pub struct Paddle {
    pub pos: Point,
    pub width: u32,
    pub height: u32,
    pub speed: f64, // pixels /p ms
    pub rect: Rect
}

impl Paddle {
    pub fn new(pos: Point, width: u32, height: u32, speed: f64) -> Paddle {
        return Paddle {
            pos: pos.clone(),
            width: width,
            height: height,
            speed: speed,
            rect: Rect::new(pos.x as i32, pos.y as i32, width, height)
        };
    }

    pub fn y(&self) -> i32 {
        self.pos.y as i32
    }

    pub fn sdl_rect(&mut self) -> &sdl2::rect::Rect {
        self.rect.set_x(self.pos.x as i32);
        self.rect.set_y(self.pos.y as i32);

        return &self.rect;
    }

    pub fn up(&mut self, delta_ms: u64, limit: f64) -> () {
        let step_size = delta_ms as f64 * self.speed;

        if (self.pos.y - step_size) >= limit {
            self.pos.y = self.pos.y - step_size;
        } else {
            // If distance to the edge is less than the step size the paddle
            // will never hit the edge. In this case just ignore speed and set
            // it manually.
            self.pos.y = 0.0;
        }
    }

    pub fn down(&mut self, delta_ms: u64, limit: f64) -> () {
        let step_size = delta_ms as f64 * self.speed;

        if (self.pos.y + self.height as f64 + step_size) <= limit {
            self.pos.y = self.pos.y + step_size;
        } else {
            self.pos.y = limit - self.height as f64;
        }
    }
}
