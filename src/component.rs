extern crate sdl2;
use sdl2::rect::Rect;

use vector::Vector;

use constants::{Color};

pub type Component<'a> = (&'a sdl2::rect::Rect, &'a Color);

#[derive(Debug)]
pub struct Ball {
    pub pos: Vector,
    pub width: u32,
    pub height: u32,
    pub velocity: Vector,
    pub rect: Rect,
    pub color: Color,
}

impl Ball {
    pub fn new(pos: Vector, width: u32, height: u32, velocity: Vector, color: Color) -> Ball {
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
            color: color,
            velocity: velocity,
        }
    }

    pub fn y(&self) -> i32 { self.pos.y as i32 }

    pub fn x(&self) -> i32 { self.pos.x as i32 }

    pub fn center(&self) -> Vector {
        Vector { x: self.pos.x + self.width as f64 / 2.0, y: self.pos.y + self.height as f64 / 2.0 }
    }

    pub fn components(&mut self, origin: &Vector) -> Vec<Component> {
        self.rect.set_x(self.pos.x as i32 + origin.x as i32);
        self.rect.set_y(self.pos.y as i32 + origin.y as i32);

        return vec![
            (&self.rect, &self.color),
        ];
    }

    pub fn update(&mut self, delta_ms: u64) -> () {
        // TODO: Try and implement edge sticking here. Naive approach is
        // regardless of whether a collision will occur detect the distance to
        // the x co-ord +/- paddle_width. If it is within some threshold round
        // up.
        //
        // This would work better if the delta_ms step size is controlled so the
        // max distance travelled in the x plane can be fixed for a loop. That
        // way the "stickyness" can be tuned.
        //
        // NOTE: make sure to factor in sign so we only stick in one direction.

        let new_x = (self.velocity.x * delta_ms as f64).round();
        let new_y = (self.velocity.y * delta_ms as f64).round();

        self.pos.add_mut(&Vector { x: new_x, y: new_y });
    }

    pub fn set_velocity_y_magnitude(&mut self, new_y_mag: f64) -> () {
        let mut new_y = new_y_mag;

        if self.velocity.y.is_sign_negative() { new_y = new_y_mag * -1.0; }

        self.velocity.y = new_y
    }

    // TODO: This should be done with generics that impl the correct traits
    pub fn distance_to(&self, paddle: &Paddle) -> Vector {
        self.center().subtract(&paddle.center())
    }

    pub fn is_moving_up(&self) -> bool {
        self.velocity.y.is_sign_negative()
    }

    pub fn is_moving_down(&self) -> bool {
        self.velocity.y.is_sign_positive()
    }

    pub fn is_moving_left(&self) -> bool {
        self.velocity.x.is_sign_negative()
    }

    pub fn is_moving_right(&self) -> bool {
        self.velocity.x.is_sign_positive()
    }

    pub fn flip_y(&mut self) -> () {
        self.velocity.y = self.velocity.y * -1.0
    }

    pub fn flip_x(&mut self) -> () {
        self.velocity.x = self.velocity.x * -1.0
    }

    pub fn top_edge(&self) -> i32 {
        self.y()
    }

    pub fn bottom_edge(&self) -> i32 {
        self.y() + self.height as i32
    }

    pub fn left_edge(&self) -> i32 {
        self.x()
    }

    pub fn right_edge(&self) -> i32 {
        self.x() + self.width as i32
    }
}

#[derive(Debug)]
pub struct Paddle {
    pub pos: Vector,
    pub width: u32,
    pub height: u32,
    pub velocity: Vector,
    pub background: Rect,
    pub border: Rect,
    pub border_color: Color,
    pub background_color: Color,
    pub segments: [(sdl2::rect::Rect, Color); 6],
    pub hits: u8,
}

impl Paddle {
    pub fn new(pos: Vector, width: u32, height: u32, speed: f64, color: Color) -> Paddle {
        let segment_height = (height - 10) / 5;

        return Paddle {
            pos: pos.clone(),
            width: width,
            height: height,
            background: Rect::new(pos.x as i32, pos.y as i32, width - 10, height - 10),
            border: Rect::new(pos.x as i32, pos.y as i32, width, height),
            velocity: Vector { x: 0.0, y: speed },
            border_color: color.clone(),
            background_color: Color::White,
            segments: [
                (Rect::new(0, 0, width - 10, segment_height), color.clone()),
                (Rect::new(0, 0, width - 10, segment_height), color.clone()),
                (Rect::new(0, 0, width - 10, segment_height), color.clone()),
                (Rect::new(0, 0, width - 10, segment_height), color.clone()),
                (Rect::new(0, 0, width - 10, segment_height), color.clone()),
                (Rect::new(0, 0, width - 10, segment_height), color.clone()),
            ],
            hits: 0,
        };
    }

    pub fn x(&self) -> i32 {
        self.pos.x as i32
    }

    pub fn y(&self) -> i32 {
        self.pos.y as i32
    }

    pub fn center(&self) -> Vector {
        Vector { x: self.pos.x + self.width as f64 / 2.0, y: self.pos.y + self.height as f64 / 2.0 }
    }

    pub fn hit(&mut self) -> () {
        if self.hits < 5 {
            self.hits = self.hits + 1;
        }
    }

    pub fn miss(&mut self) -> () {
        if self.hits > 0 {
            self.hits = self.hits - 1;
        }
    }

    pub fn reset_hits(&mut self) -> () {
        self.hits = 0;
    }

    pub fn set_color(&mut self, color: &Color) -> () {
        self.border_color = color.clone();

        for segement in self.segments.iter_mut() {
            segement.1 = color.clone();
        }
    }

    pub fn components(&mut self, origin: &Vector) -> Vec<Component> {
        let segment_height = (self.height - 10) / 5;

        self.background.set_x((self.pos.x + origin.x + 5.0) as i32);
        self.background.set_y((self.pos.y + origin.y + 5.0) as i32);

        self.border.set_x((self.pos.x + origin.x) as i32);
        self.border.set_y((self.pos.y + origin.y) as i32);

        let mut components = vec![
            (&self.border, &self.border_color),
            (&self.background, &self.background_color),
        ];

        for i in 0..self.hits {
            self.segments[i as usize].0.set_x((self.pos.x + origin.x + 5.0) as i32);
            self.segments[i as usize].0.set_y((self.pos.y + (self.height - segment_height * (i as u32 + 1)) as f64 + origin.y - 5.0) as i32);

        }

        for i in 0..self.hits {
            components.push((&self.segments[i as usize].0, &self.segments[i as usize].1));
        }

        return components;
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

    //pub fn left_edge(&self) -> i32 {
    //    self.x()
    //}

    pub fn right_edge(&self) -> i32 {
        self.x() + self.width as i32
    }

    pub fn top_edge(&self) -> i32 {
        self.y()
    }

    pub fn bottom_edge(&self) -> i32 {
        self.y() + self.height as i32
    }
}
