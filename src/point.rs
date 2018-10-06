#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    pub fn length(&self) -> f64 {
        ((self.x).powi(2) + (self.y).powi(2)).sqrt()
    }

    pub fn subtract(&self, other: &Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
    }

    pub fn add_mut(&mut self, other: &Point) -> () {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }

    pub fn normalize(self) -> Point {
        let length = self.length();

        Point { x: self.x / length, y: self.y / length }
    }
}
