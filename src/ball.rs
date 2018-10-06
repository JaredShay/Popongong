extern crate sdl2;
use sdl2::rect::Rect;

use point::Point;

#[derive(Debug)]
pub struct Ball {
    pub pos: Point,
    pub rect: Rect,
    pub speed: f64, // pixels / ms
    pub movement_vector: Point
}

impl Ball {
    pub fn new(pos: Point, target: Point, width: u32, height: u32) -> Ball {
        return Ball {
            pos: pos.clone(),
            rect: Rect::new(
                pos.x as i32,
                pos.y as i32,
                width,
                height
            ),
            speed: 0.5,
            movement_vector: target.subtract(&pos).normalize()
        }
    }

    pub fn x(&self) -> i32 {
        self.pos.x as i32
    }

    pub fn y(&self) -> i32 {
        self.pos.y as i32
    }

    pub fn update(&mut self, delta_ms: u64) -> () {
        let new_x = (self.movement_vector.x * self.speed * delta_ms as f64).round();
        let new_y = (self.movement_vector.y * self.speed * delta_ms as f64).round();

        self.pos.add_mut(&Point { x: new_x, y: new_y });
    }

    pub fn sdl_rect(&mut self) -> &sdl2::rect::Rect {
        self.rect.set_x(self.pos.x as i32);
        self.rect.set_y(self.pos.y as i32);

        return &self.rect;
    }
}
