use std::io;
use std::io::Write;

pub fn read_f64(msg : &str) -> f64 {
    let mut input = String::new();

    print!("{}", msg);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Error de lectura");

    let num: f64 = input.trim().parse().expect("Por favor, digite un numero.");
    
    return num;
}

pub fn read_i32(msg : &str) -> i32 {
    let mut input = String::new();

    print!("{}", msg);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Error de lectura");

    let num: i32 = input.trim().parse().expect("Por favor, digite un numero.");
    
    return num;
}

pub fn read_String(msg : &str) -> String {
    let mut input = String::new();

    print!("{}", msg);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Error de lectura");

    return input.trim().to_string();
}
