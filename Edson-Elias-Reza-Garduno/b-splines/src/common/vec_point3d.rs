pub use std::vec::Vec;
pub use crate::common::point3d::Point3D;
use std::ops::Range;

pub fn new() -> Vec<Point3D> {
    let v:Vec<Point3D> = Vec::new();
    v
}

pub trait Transform {
    fn to_string(&self) -> String;
    fn to_tuples(&self) -> Vec<(f64, f64, f64)>;
}

impl Transform for Vec<Point3D> {
    fn to_string(&self) -> String {
        let mut str = String::new();

        for i in 0..self.len() {
            let point = self.get(i);

            str = format!("{}{}", str, point.expect("No hay puntos de control").to_string());

            if i < self.len() - 1 {
                str = format!("{},", str);
            }
        }

        str
    }

    fn to_tuples(&self) -> Vec<(f64, f64, f64)> {
        let mut v = vec![];

        for point in self {
            v.push((point.x, point.y, point.z));
        }

        return v;
    }
}

pub trait RangeXY {
    fn x_range(&self) -> Range<f64>;
    fn y_range(&self) -> Range<f64>;
    fn z_range(&self) -> Range<f64>;
}

impl RangeXY for Vec<Point3D> {
    fn x_range(&self) -> Range<f64> {
        let mut min_x:f64 = 0.0;
        let mut max_x:f64 = 0.0;

        for point in self {
            if point.x < min_x {
                min_x = point.x;
            }

            if point.x > max_x {
                max_x = point.x;
            }
        }

        Range {start: min_x, end: max_x}
    }

    fn y_range(&self) -> Range<f64> {
        let mut min_y:f64 = 0.0;
        let mut max_y:f64 = 0.0;

        for point in self {
            if point.y < min_y {
                min_y = point.y;
            }

            if point.y > max_y {
                max_y = point.y;
            }
        }

        Range {start: min_y, end: max_y}
    }

    fn z_range(&self) -> Range<f64> {
        let mut min_z:f64 = 0.0;
        let mut max_z:f64 = 0.0;

        for point in self {
            if point.z < min_z {
                min_z = point.z;
            }

            if point.z > max_z {
                max_z = point.z;
            }
        }

        Range {start: min_z, end: max_z}
    }
}