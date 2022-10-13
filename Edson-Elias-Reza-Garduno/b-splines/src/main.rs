use b_splines::common::point::Point;
use b_splines::common::read::read_loop;
use b_splines::graphic::curves as curves;
use b_splines::graphic::plotting as plot;
use b_splines::common::read;
use b_splines::common::verification;

fn main() {
 
    //* This is an example of the book "The NURBS Book" by Piegl and Tiller.

    // Knot vector, must be always increasing
    //let knots:Vec<f64> = vec![0.0,0.0,0.0,0.125,0.25,0.375,0.5,0.625,0.75,0.875,1.0,1.0,1.0];

    // Control points
    // let points:Vec<Point> = vec![
    //     Point::new(0.0, 5.0),
    //     Point::new(1.0, 7.0),
    //     Point::new(2.0, 5.0),
    //     Point::new(4.0, 10.0),
    //     Point::new(8.0, 9.0),
    //     Point::new(10.0, 0.0),
    //     Point::new(14.0, 5.0),
    //     Point::new(15.0, 3.0),
    //     Point::new(19.0, 8.0),
    //     Point::new(20.0, 7.0)
    // ];

    // Degree
    //let p:i32 = 2;

    // Generate the curve
    //let bspline: Vec<Point> = curves::bspline(&knots, &points, p, 0.0001);

    //let mut name:String = "name".to_owned();
    //name.push_str(".png");
    // Draw the curve
    //plot::plot_curve("Curva",&name, &points, &bspline, 1200,800);
    
    // * Aqui comienza la lectura para el usuario

    // Lectura de nudos
    print!("Ingrese el numero de nudos: ");
    let n:i32 = read::read_unique();
    verification::mayor_que(n as f64, 2.0, "El numero de nudos debe ser mayor que 2");
    println!("Ingrese los nudos: ");
    let knots:Vec<f64> = read_loop(n, "nudo");
    verification::nodos_validos(&knots);
    verification::valores_validos(&knots, 0.0, 1.0);

    // Lectura de puntos de control
    print!("Ingrese el numero de puntos de control: ");
    let n:i32 = read::read_unique();
    verification::mayor_que(n as f64, 2.0, "El numero de puntos de control debe ser mayor que 2");
    let points:Vec<Point> = read::read_point_loop(n);

    // Lectura del grado
    print!("Ingrese el grado: ");
    let p:i32 = read::read_unique();
    verification::mayor_que(p as f64, 1.0, "El grado debe ser mayor que 1");

    // Lectura de saltos en la curva, define la resolucion de la curva
    print!("Ingrese el salto en u: ");
    let u:f64 = read::read_unique();
    verification::mayor_que(u, 0.0, "El salto en u debe ser mayor que 0");

    // Generacion de la curva
    let bspline: Vec<Point> = curves::bspline(&knots, &points, p, u);

    // Dibujo de la curva
    print!("Ingrese el nombre del archivo para guardar la curva (.png): ");
    let mut name: String = read::read_unique();
    name = verification::nombre_con_extension(&name, ".png");

    plot::plot_curve("Curva",&name, &points, &bspline, 1200,800);
}
