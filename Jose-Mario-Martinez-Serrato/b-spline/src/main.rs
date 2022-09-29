fn main(){
    
    println!("Valores de U");
    let mut U : String = String::new(); //not mutable, stores the number of values of u-vector
    std::io::stdin().read_line(&mut U).unwrap(); //read console line
    let u: i8 = U.trim().parse().unwrap();

    for x in 0..u{
        println!("{}",u);

    }

}