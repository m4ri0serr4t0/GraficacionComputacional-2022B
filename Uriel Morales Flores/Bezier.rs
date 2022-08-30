//URIEL MORALES FLORES//
//Primer programa//
//16-08-2022//
//Evaluar un polinomio usando Metodo de Horner//

use std::io;
use std::marker::Sized;

fn main() {

    let mut x = 500;

    println!("Ingrese el valor1: ");
    let mut n1 = String::new();
    io::stdin().read_line(&mut n1);
    let n1 = n1.trim();
    let n1: i32 = n1.parse().unwrap();

    println!("Ingrese el valor 2: ");
    let mut n2 = String::new();
    io::stdin().read_line(&mut n2);
    let n2 = n2.trim();
    let n2: i32 = n2.parse().unwrap();

    println!("Ingrese el valor 3: ");
    let mut n3 = String::new();
    io::stdin().read_line(&mut n3);
    let n3 = n3.trim();
    let n3: i32 = n3.parse().unwrap();

    println!("Ingrese el valor 4: ");
    let mut n4 = String::new();
    io::stdin().read_line(&mut n4);
    let n4 = n4.trim();
    let n4: i32 = n4.parse().unwrap();

    println!("Ingrese el valor 5: ");
    let mut n5 = String::new();
    io::stdin().read_line(&mut n5);
    let n5 = n5.trim();
    let n5: i32 = n5.parse().unwrap();

    let arreglo = [n1, n2, n3, n4, n5];

    let mut resultado = 0;

    //let mut i = 0;
    //let i = i.len();

    for i in 0..arreglo.len(){
        resultado = resultado * x + arreglo[i];

    }

    println!("Resultado es {}", resultado);



}
