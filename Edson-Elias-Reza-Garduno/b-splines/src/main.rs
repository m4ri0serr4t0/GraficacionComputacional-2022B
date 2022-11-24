use b_splines::common::point::Point;
use b_splines::common::read;
use b_splines::common::read::read_vector;
use b_splines::common::verification;
use b_splines::graphic::bspline;
use b_splines::graphic::plotting as plot;

fn main() {
    //* This is an example of the book "The NURBS Book" by Piegl and Tiller.

    // * Aqui comienza la lectura para el usuario

    println!("Bienvenido al programa de generación de B-Splines");
    println!("Seleccione la opción que desea realizar");
    println!("1. Generar una curva");
    println!("2. Generar el abecedario");
    println!("Otro. Salir");
    print!("Opción: ");
    let opcion: i8 = read::read();

    match opcion {
        1 => curve(),
        2 => generar_abc(),
        _ => println!("Hasta luego"),
    }
}

fn curve() {
    print!("Ingrese el numero de nudos: ");
    let n: i32 = read::read();

    // Lectura de nudos
    println!("Ingrese los nudos: ");
    let knots: Vec<f64> = read_vector(n);

    // Lectura de puntos de control
    print!("Ingrese el numero de puntos de control: ");
    let n: i32 = read::read();
    let points: Vec<Point> = read::read_vector(n);

    // Lectura del grado
    print!("Ingrese el grado: ");
    let p: i32 = read::read();

    // Lectura de saltos en la curva, define la resolucion de la curva
    let u = 0.001;

    // Generacion de la curva
    let bspline: Vec<Point> = bspline::bspline(&knots, &points, p, u);

    // Dibujo de la curva
    print!("Ingrese el nombre del archivo para guardar el resultado (.png): ");
    let mut name: String = read::read();
    name = verification::nombre_con_extension(&name, ".png");

    plot::plot_curve(&name, &points, &bspline);
}

fn generar_abc() {
    let p:i32 = 2;
    //* Letra A
    // let a:Vec<Point> = vec![
    //     Point::new(6.0,4.0),
    //     Point::new(1.0, 1.0),
    //     Point::new(4.0, 12.0),
    //     Point::new(7.0, 1.0),
    //     Point::new(6.0, 1.0),
    //     Point::new(4.0, 10.0),
    //     Point::new(2.0, 1.0),
    //     Point::new(1.0, 1.0),
    //     Point::new(6.0,4.0),
    // ];

    // * Letra B :(

    // * Letra C
    // let letter: Vec<Point> = vec![
    //     Point::new(7.0, 7.0),
    //     Point::new(3.0, 7.0),
    //     Point::new(1.0, 4.0),
    //     Point::new(3.0, 1.0),
    //     Point::new(7.0, 1.0),
    //     Point::new(7.0, 2.0),
    //     Point::new(3.0, 4.0),
    //     Point::new(4.0, 7.0),
    //     Point::new(7.0, 6.0),
    //     Point::new(7.0, 7.0),
    // ];

    // * Letra D
    // let letter:Vec<Point> = vec![
    //     Point::new(1.0, 1.0),
    //     Point::new(1.0, 7.0),
    //     Point::new(7.0, 6.0),
    //     Point::new(10.0, 4.0),
    //     Point::new(7.0, 2.0),
    //     Point::new(1.0, 1.5),
    //     Point::new(6.5, 3.0),
    //     Point::new(8.0, 4.0),
    //     Point::new(6.0, 5.5),
    //     Point::new(2.0, 6.0),
    //     Point::new(1.0, 1.0)
    // ];

    // * Letra E
    // let letter:Vec<Point> = vec![
    //     Point::new(1.0, 1.0),
    //     Point::new(1.0, 8.0),
    //     Point::new(5.0, 8.0),
    //     Point::new(5.0, 7.0),
    //     Point::new(1.5, 7.0),
    //     Point::new(1.5, 5.0),
    //     Point::new(4.0, 5.0),
    //     Point::new(4.0, 4.0),
    //     Point::new(1.5, 4.0),
    //     Point::new(1.5, 2.0),
    //     Point::new(5.0, 2.0),
    //     Point::new(5.0, 1.0),
    //     Point::new(1.0, 1.0),
    // ];

    // * Letra F
    // let letter:Vec<Point> = vec![
    //     Point::new(1.0, 1.0),
    //     Point::new(1.0, 8.0),
    //     Point::new(5.0, 8.0),
    //     Point::new(5.0, 7.0),
    //     Point::new(1.5, 7.0),
    //     Point::new(1.5, 5.0),
    //     Point::new(4.0, 5.0),
    //     Point::new(4.0, 4.0),
    //     Point::new(1.5, 4.0),
    //     Point::new(1.5, 1.0),
    //     Point::new(1.0, 1.0),
    // ];

    // * Letra G
    // let letter:Vec<Point> = vec![
    //     Point::new(7.0, 7.0),
    //     Point::new(2.0, 7.0),
    //     Point::new(1.0, 4.0),
    //     Point::new(2.0, 2.0),
    //     Point::new(5.0, 1.0),
    //     Point::new(7.0, 2.0),
    //     Point::new(7.0, 4.0),
    //     Point::new(5.0, 4.0),
    //     Point::new(6.0, 3.0),
    //     Point::new(7.0, 3.0),
    //     Point::new(1.0, 4.0),
    //     Point::new(7.0, 7.0),
    // ];

    // * Letra H
    // let letter:Vec<Point> = vec![
    //     Point::new(1.0, 7.0),
    //     Point::new(1.0, 1.0),
    //     Point::new(2.0, 1.0),
    //     Point::new(2.0, 3.0),
    //     Point::new(5.0, 3.0),
    //     Point::new(5.0, 1.0),
    //     Point::new(6.0, 1.0),
    //     Point::new(6.0, 7.0),
    //     Point::new(5.0, 7.0),
    //     Point::new(5.0, 4.0),
    //     Point::new(2.0, 4.0),
    //     Point::new(2.0, 7.0),
    //     Point::new(1.0, 7.0),
    // ];

    // * Letra I
    // let letter:Vec<Point> = vec![
    //     Point::new(1.0, 7.0),
    //     Point::new(1.0, 1.0),
    //     Point::new(2.0, 1.0),
    //     Point::new(2.0, 7.0),
    //     Point::new(1.0, 7.0),
    // ];

    // * Letra J
    // let letter:Vec<Point> = vec![
    //     Point::new(1.0, 1.0),
    //     Point::new(4.0, 1.0),
    //     Point::new(4.0, 7.0),
    //     Point::new(3.0, 7.0),
    //     Point::new(3.0, 2.0),
    //     Point::new(1.0, 1.0),
    // ];

    // * Letra K

    // * Letra L
    // let letter:Vec<Point> = vec![
    //     Point::new(1.0, 7.0),
    //     Point::new(1.0, 1.0),
    //     Point::new(4.0, 1.0),
    //     Point::new(4.0, 2.0),
    //     Point::new(2.0, 2.0),
    //     Point::new(2.0, 7.0),
    //     Point::new(1.0, 7.0),
    // ];

    // * Letra M
    // let letter:Vec<Point> = vec![
    //     Point::new(1.0, 6.0),
    //     Point::new(3.0, 6.5),
    //     Point::new(4.0, 5.0),
    //     Point::new(7.0, 7.0),
    //     Point::new(7.0, 1.0),
    //     Point::new(6.0, 1.0),
    //     Point::new(6.0, 6.0),
    //     Point::new(4.0, 4.0),
    //     Point::new(2.0, 6.0),
    //     Point::new(2.0, 1.0),
    //     Point::new(1.0, 1.0),
    //     Point::new(1.0, 6.0),
    // ];

    // * Letra N
    // let letter:Vec<Point> = vec![
    //     Point::new(1.0, 1.0),
    //     Point::new(1.0, 7.0),
    //     Point::new(2.0, 7.0),
    //     Point::new(5.0, 2.0),
    //     Point::new(5.0, 7.0),
    //     Point::new(6.0, 7.0),
    //     Point::new(7.0, 1.0),
    //     Point::new(6.0, 1.0),
    //     Point::new(2.0, 6.0),
    //     Point::new(2.0, 1.0),
    //     Point::new(1.0, 1.0),
    // ];

    // * Letra O

    // * Letra P
    
    // * Letra Q

    // * Letra R

    // * Letra S
    // let letter:Vec<Point> = vec![
    //     Point::new(6.0, 7.0),
    //     Point::new(6.0, 6.0),
    //     Point::new(2.0, 6.0),
    //     Point::new(2.0, 4.0),
    //     Point::new(6.0, 4.0),
    //     Point::new(6.0, 1.0),
    //     Point::new(1.0, 1.0),
    //     Point::new(1.0, 2.0),
    //     Point::new(5.0, 2.0),
    //     Point::new(5.0, 3.0),
    //     Point::new(1.0, 3.0),
    //     Point::new(1.0, 7.0),
    //     Point::new(6.0, 7.0),
    // ];

    // * Letra T
    // let letter:Vec<Point> = vec![
    //     Point::new(4.0, 7.0),
    //     Point::new(7.0, 7.0),
    //     Point::new(7.0, 6.0),
    //     Point::new(4.5, 6.0),
    //     Point::new(4.5, 1.0),
    //     Point::new(3.5, 1.0),
    //     Point::new(3.5, 6.0),
    //     Point::new(1.0, 6.0),
    //     Point::new(1.0, 7.0),
    //     Point::new(4.0, 7.0),
    // ];

    // * Letra U
    // let letter:Vec<Point> = vec![
    //     Point::new(1.0, 7.0),
    //     Point::new(1.0, 1.0),
    //     Point::new(4.0, 1.0),
    //     Point::new(7.0, 1.0),
    //     Point::new(7.0, 7.0),
    //     Point::new(6.0, 7.0),
    //     Point::new(6.0, 2.0),
    //     Point::new(2.0, 2.0),
    //     Point::new(2.0, 7.0),
    //     Point::new(1.0, 7.0),
    // ];

    // * Letra V
    // let letter:Vec<Point> = vec![
    //     Point::new(2.0, 7.0),
    //     Point::new(4.0, 2.0),
    //     Point::new(6.0, 7.0),
    //     Point::new(7.0, 7.0),
    //     Point::new(4.0, 0.0),
    //     Point::new(1.0, 7.0),
    //     Point::new(2.0, 7.0),
    // ];

    // * Letra W
    // let letter:Vec<Point> = vec![
    //     Point::new(1.0, 7.0),
    //     Point::new(1.0, 1.0),
    //     Point::new(4.0, 5.0),
    //     Point::new(7.0, 1.0),
    //     Point::new(7.0, 7.0),
    //     Point::new(6.0, 7.0),
    //     Point::new(6.0, 2.0),
    //     Point::new(4.0, 6.0),
    //     Point::new(2.0, 2.0),
    //     Point::new(2.0, 7.0),
    //     Point::new(1.0, 7.0),
    // ];

    // * Letra X 

    // * Letra Y
    // let letter:Vec<Point> = vec![
    //     Point::new(2.0, 7.0),
    //     Point::new(4.0, 5.0),
    //     Point::new(6.0, 7.0),
    //     Point::new(7.0, 7.0),
    //     Point::new(4.5, 4.0),
    //     Point::new(4.5, 1.0),
    //     Point::new(3.5, 1.0),
    //     Point::new(3.5, 4.0),
    //     Point::new(1.0, 7.0),
    //     Point::new(2.0, 7.0),
    // ];

    // * Letra Z
    // let letter:Vec<Point> = vec![
    //     Point::new(1.0, 7.0),
    //     Point::new(7.0, 7.0),
    //     Point::new(7.0, 6.0),
    //     Point::new(2.0, 2.0),
    //     Point::new(7.0, 2.0),
    //     Point::new(7.0, 1.0),
    //     Point::new(1.0, 1.0),
    //     Point::new(1.0, 2.0),
    //     Point::new(5.0, 6.0),
    //     Point::new(1.0, 6.0),
    //     Point::new(1.0, 7.0),        
    // ];

    // * Numero 0

    // * Numero 1
    // let letter:Vec<Point> = vec![
    //     Point::new(1.0, 6.0),
    //     Point::new(2.0, 7.0),
    //     Point::new(4.0, 7.0),
    //     Point::new(4.0, 1.0),
    //     Point::new(2.0, 1.0),
    //     Point::new(2.0, 6.0),
    //     Point::new(1.0, 6.0),
    // ];

    // * Numero 2
    // let letter:Vec<Point> = vec![
    //     Point::new(1.0, 5.0),
    //     Point::new(1.0, 7.0),
    //     Point::new(5.0, 7.0),
    //     Point::new(5.0, 6.0),
    //     Point::new(2.0, 2.0),
    //     Point::new(5.0, 2.0),
    //     Point::new(5.0, 1.0),
    //     Point::new(1.0, 1.0),
    //     Point::new(1.0, 2.0),
    //     Point::new(5.0, 6.0),
    //     Point::new(1.0, 6.0),
    //     Point::new(1.0, 5.0),        
    // ];

    // * Numero 3
    // let letter:Vec<Point> = vec![
    //     Point::new(1.0, 7.0),
    //     Point::new(7.0, 7.0),
    //     Point::new(7.0, 5.0),
    //     Point::new(3.0, 4.0),
    //     Point::new(7.0, 4.0),
    //     Point::new(7.0, 2.0),
    //     Point::new(1.0, 2.0),
    //     Point::new(1.0, 4.0),
    //     Point::new(5.0, 5.0),
    //     Point::new(1.0, 6.0),
    //     Point::new(1.0, 7.0),
    // ];


    // * Numero 4
    // let letter:Vec<Point> = vec![
    //     Point::new(7.0, 7.0),
    //     Point::new(7.0, 1.0),
    //     Point::new(6.0, 1.0),
    //     Point::new(6.0, 4.0),
    //     Point::new(1.0, 4.0),
    //     Point::new(1.0, 7.0),
    //     Point::new(2.0, 7.0),
    //     Point::new(2.0, 5.0),
    //     Point::new(6.0, 5.0),
    //     Point::new(6.0, 7.0),
    //     Point::new(7.0, 7.0),
    // ];

    // * Numero 5

    // * Numero 6

    // * Numero 7
    let letter:Vec<Point> = vec![
        Point::new(1.0, 7.0),
        Point::new(7.0, 7.0),
        Point::new(7.0, 6.0),
        Point::new(2.0, 1.0),
        Point::new(1.0, 1.0),
        Point::new(1.0, 2.0),
        Point::new(6.0, 6.0),
        Point::new(1.0, 6.0),
        Point::new(1.0, 7.0),
    ];

    // * Numero 8

    // * Numero 9

    let mut knots: Vec<f64> = vec![];

    let size: i32 = ((p + 1 + letter.len() as i32) - (2 * (p + 1))) + 1;

    for _ in 0..p + 1 {
        knots.push(0.0);
    }

    for i in 1..size {
        knots.push(i as f64 * (1.0 / size as f64))
    }

    for _ in 0..p + 1 {
        knots.push(1.0);
    }

    let bspline = bspline::bspline(&knots, &letter, p, 0.001);
    plot::plot_polygon("results/font/7.png", &letter, &bspline);
}