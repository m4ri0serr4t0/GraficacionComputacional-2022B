use std::fmt;
use std::str::FromStr;
use std::ops::{Add,Mul};
use std::cmp::{Eq,PartialEq};

#[derive(Clone, Copy)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.4},{:.4},{:.4})", self.x, self.y,self.z)
    }
}

impl FromStr for Point3D {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x:f64 = parts.next().unwrap().parse::<f64>().map_err(|_| "Invalid x value".to_string())?;
        let y:f64 = parts.next().unwrap().parse::<f64>().map_err(|_| "Invalid y value".to_string())?;
        let z:f64 = parts.next().unwrap().parse::<f64>().map_err(|_| "Invalid z value".to_string())?;
        Ok(Point3D { x, y, z })
    }
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        Point3D { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Mul<f64> for Point3D {
    type Output = Point3D;

    fn mul(self, other: f64) -> Point3D {
        Point3D { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }
}

impl Eq for Point3D {}