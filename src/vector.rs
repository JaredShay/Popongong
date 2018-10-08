#[derive(Debug, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64
}

impl Vector {
    pub fn length(&self) -> f64 {
        ((self.x).powi(2) + (self.y).powi(2)).sqrt()
    }

    pub fn subtract(&self, other: &Vector) -> Vector {
        Vector { x: self.x - other.x, y: self.y - other.y }
    }

    pub fn add_mut(&mut self, other: &Vector) -> () {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }

    pub fn product(&self, value: f64) -> Vector {
        Vector { x: self.x * value, y: self.y * value }
    }

    pub fn normalize(self) -> Vector {
        let length = self.length();

        Vector { x: self.x / length, y: self.y / length }
    }
}
