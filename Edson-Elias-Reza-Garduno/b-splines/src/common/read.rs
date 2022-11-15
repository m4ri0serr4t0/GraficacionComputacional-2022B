use std::{fmt::Debug, str::FromStr, io::{stdin, stdout, Write}};

use super::vec_point3d::Point3D;

pub fn read_vector<T: std::str::FromStr>(n:i32) -> Vec<T> where <T as FromStr>::Err: Debug {
    let mut values:Vec<T> = Vec::new();
    for i in 0..n {
        let mut input:String = String::new();
        print!("Ingrese el elemento {}: ", i+1);
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let value:T = input.trim().parse::<T>().unwrap();            
        values.push(value);
    }
    return values;
}

pub fn read<T: std::str::FromStr>() -> T where <T as FromStr>::Err: Debug {
    let mut input:String = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    let value:T = input.trim().parse::<T>().unwrap();
    return value;
}

pub fn read_matrix(n:i32, m:i32) -> Vec<Vec<Point3D>> {
    let mut matrix:Vec<Vec<Point3D>> = Vec::new();
    for i in 0..n {
        let mut row:Vec<Point3D> = Vec::new();
        for j in 0..m {
            let mut input:String = String::new();
            print!("Ingrese el elemento ({},{}) de la matriz: ", i + 1, j + 1);
            stdout().flush().unwrap();
            stdin().read_line(&mut input).unwrap();
            let value:Point3D = input.trim().parse::<Point3D>().unwrap();
            row.push(value);
        }
        matrix.push(row);
    }
    return matrix;
}