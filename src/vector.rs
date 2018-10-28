#[derive(Debug, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64
}

impl Vector {
    pub fn subtract(&self, other: &Vector) -> Vector {
        Vector { x: self.x - other.x, y: self.y - other.y }
    }

    pub fn add_mut(&mut self, other: &Vector) -> () {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}
