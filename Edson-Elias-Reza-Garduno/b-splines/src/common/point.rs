use std::fmt;

pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.4},{:.4})", self.x, self.y)
    }
}
