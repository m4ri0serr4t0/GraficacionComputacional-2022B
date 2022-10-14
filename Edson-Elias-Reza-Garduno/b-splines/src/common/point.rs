use std::fmt;
use std::str::FromStr;
use std::ops::{Add,Mul};
use std::cmp::{Eq,PartialEq};

#[derive(Clone, Copy)]
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

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse::<f64>().map_err(|_| "Invalid x value".to_string())?;
        let y = parts.next().unwrap().parse::<f64>().map_err(|_| "Invalid y value".to_string())?;
        Ok(Point { x, y })
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, other: f64) -> Point {
        Point { x: self.x * other, y: self.y * other }
    }
}

impl Mul<Point> for Point {
    type Output = f64;

    fn mul(self, other: Point) -> f64 {
        return self.x * other.x + self.y * other.y;
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Eq for Point {}

// Test 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        let p3 = p1 + p2;
        assert_eq!(p3.x, 4.0);
        assert_eq!(p3.y, 6.0);

        let p4 = p1 * p2;
        assert_eq!(p4, 11.0);

        let p5 = p1 * 2.0;
        assert_eq!(p5.x, 2.0);
        assert_eq!(p5.y, 4.0);

        let p6 = Point::from_str("1.0,2.0").unwrap();
        assert_eq!(p6.x, 1.0);
        assert_eq!(p6.y, 2.0);

        assert_eq!(p1==p2, false);
        assert_eq!(p1==p6, true);

    }
}