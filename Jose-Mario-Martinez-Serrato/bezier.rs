fn main(){

    let mut u : String = String::new(); //Declaring u and n as a new String
    let mut n : String = String::new();

    println!("Curvas de Bezier");

    println!("Ingrese el valor de u: ");
    std::io::stdin().read_line(&mut u).unwrap(); //Reading u from console line, storing the value in another variable


    let u_int: i8 = u.trim().parse().unwrap(); //Parsing u into a integer
    println!("El valor de u es {}", u);

    println!("Ingrese el valor de n: ");
    std::io::stdin().read_line(&mut n).unwrap();

    let n_int: i8 = n.trim().parse().unwrap();
    println!("El valor de n es {}", n);



}

fn bezier(){

}

fn factorial(x: i32){
    

}

fn puntosControl() -> i32{
    let mut x : String = String::new();
    let mut y : String = String::new();
    println!("X: ")
    std::io::stdin().read_line(&mut x).unwrap(); 
    println!("Y: ")
    std::io::stdin().read_line(&mut y).unwrap(); 

}