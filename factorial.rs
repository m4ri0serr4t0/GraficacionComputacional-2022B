use std::io;
fn main() {
    
    println!("Ingrese el numero para calcular su factorial: ");
    let mut n = String::new();
    io::stdin().read_line(&mut n);
    let n = n.trim();
    let n: u32 = n.parse().unwrap();

    //let mut c = String::new();
    let mut fac = 1;
    //let mut i = 1;
    
    
    for i in 1..n {  
        

        fac = fac * (i+1);
        
        //println!("EL factorial del numero {} * {} es: {}   ",n ,i, fac);
       // println!("EL factorial del numero {} es:  ",i);
    }
    println!("EL factorial del numero {} es: {}",n, fac);

    
    }
