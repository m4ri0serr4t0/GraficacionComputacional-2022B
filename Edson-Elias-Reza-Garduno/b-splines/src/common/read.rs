use std::{fmt::Debug, str::FromStr, io::{stdin, stdout, Write}};

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

pub fn read<T: std::str::FromStr>() -> T where <T as FromStr>::Err: Debug {
    let mut input:String = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    let value:T = input.trim().parse::<T>().unwrap();
    return value;
}