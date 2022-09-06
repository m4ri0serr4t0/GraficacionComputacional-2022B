use std::fmt;

pub struct Point {
    pub x:f64,
    pub y:f64
}

impl Point {
    pub fn new(x:f64, y:f64) -> Point {
        Point {x:x, y:y}
    }

    pub fn mul(n:f64, point: &Self) -> Self {
        Self::new(n * point.x, n * point.y)
    }

    pub fn sum(point_1: &Self, point_2: &Self) -> Self {
        Self::new(point_1.x + point_2.x, point_1.y + point_2.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}
