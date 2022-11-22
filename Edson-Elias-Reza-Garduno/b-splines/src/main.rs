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
    println!("Otro. Salir");
    print!("Opción: ");
    let opcion: i8 = read::read();

    match opcion {
        1 => curve(),
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
