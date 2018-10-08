extern crate sdl2;
use sdl2::rect::Rect;

use vector::Vector;

#[derive(Debug)]
pub struct Ball {
    pub pos: Vector,
    pub width: u32,
    pub height: u32,
    pub velocity: Vector,
    pub rect: Rect,
}

impl Ball {
    pub fn new(pos: Vector, target: Vector, width: u32, height: u32, speed: f64) -> Ball {
        return Ball {
            pos: pos.clone(),
            width: width,
            height: height,
            rect: Rect::new(
                pos.x as i32,
                pos.y as i32,
                width,
                height
            ),
            velocity: target.subtract(&pos).normalize().product(speed)
        }
    }

    pub fn y(&self) -> i32 { self.pos.y as i32 }

    pub fn x(&self) -> i32 { self.pos.x as i32 }

    pub fn sdl_rect(&mut self) -> &sdl2::rect::Rect {
        self.rect.set_x(self.pos.x as i32);
        self.rect.set_y(self.pos.y as i32);

        return &self.rect;
    }

    pub fn update(&mut self, delta_ms: u64) -> () {
        let new_x = (self.velocity.x * delta_ms as f64).round();
        let new_y = (self.velocity.y * delta_ms as f64).round();

        self.pos.add_mut(&Vector { x: new_x, y: new_y });
    }
}

#[derive(Debug)]
pub struct Paddle {
    pub pos: Vector,
    pub width: u32,
    pub height: u32,
    pub velocity: Vector,
    pub rect: Rect,
}

impl Paddle {
    pub fn new(pos: Vector, width: u32, height: u32, speed: f64) -> Paddle {
        return Paddle {
            pos: pos.clone(),
            width: width,
            height: height,
            rect: Rect::new(pos.x as i32, pos.y as i32, width, height),
            velocity: Vector { x: 0.0, y: speed }
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
        let step_size = delta_ms as f64 * self.velocity.y;

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
        let step_size = delta_ms as f64 * self.velocity.y;

        if (self.pos.y + self.height as f64 + step_size) <= limit {
            self.pos.y = self.pos.y + step_size;
        } else {
            self.pos.y = limit - self.height as f64;
        }
    }
}
