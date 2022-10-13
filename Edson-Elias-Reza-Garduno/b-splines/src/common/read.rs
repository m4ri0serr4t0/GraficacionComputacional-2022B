use std::{fmt::Debug, str::FromStr, io::{stdin, stdout, Write}};
use crate::common::point::Point;

pub fn read_loop<T: std::str::FromStr>(n:i32, msg:&str) -> Vec<T> where <T as FromStr>::Err: Debug {
    let mut values:Vec<T> = Vec::new();
    for i in 0..n {
        let mut input:String = String::new();
        print!("Ingrese el {}-esimo {}: ", i + 1, msg);
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let value:T = input.trim().parse::<T>().unwrap();            
        values.push(value);
    }
    return values;
}

pub fn read_unique<T: std::str::FromStr>() -> T where <T as FromStr>::Err: Debug {
    let mut input:String = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    let value:T = input.trim().parse::<T>().unwrap();
    return value;
}

pub fn read_point_loop(n:i32) -> Vec<Point> {
    let mut points:Vec<Point> = Vec::new();
    for i in 0..n {
        print!("Ingrese el {}-esimo punto: \n", i + 1);
        print!("x: ");
        let x:f64 = read_unique();
        print!("y: ");
        let y:f64 = read_unique();
        let point:Point = Point { x, y };
        points.push(point);
    }
    return points;
}