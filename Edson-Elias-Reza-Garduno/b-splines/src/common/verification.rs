pub fn mayor_que(n:f64, min:f64, msg:&str) {
    if n < min {
        panic!("{}", msg);
    }
}

pub fn nodos_validos(knots:&Vec<f64>) {
    let n = knots.len() as i32;
    let mut i = 0;
    while i < n-1 {
        if knots[(i) as usize] > knots[(i+1) as usize] {
            panic!("Los nudos deben ser siempre crecientes");
        }
        i += 1;
    }
}

pub fn valores_validos(vector:&Vec<f64>,min:f64,max:f64) {
    let n = vector.len() as i32;
    let mut i = 0;
    while i < n {
        if vector[(i) as usize] < min || vector[(i) as usize] > max {
            panic!("Los valores deben estar entre {} y {}",min,max);
        }
        i += 1;
    }
}

pub fn nombre_con_extension(nombre:&str, extension:&str) -> String {
    let mut nombre = nombre.to_string();
    if !nombre.ends_with(extension) {
        nombre.push_str(extension);
    }
    nombre
}