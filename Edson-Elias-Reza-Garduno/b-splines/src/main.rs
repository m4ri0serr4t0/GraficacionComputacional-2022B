use b_splines::common::point::Point;
use b_splines::common::read;
use b_splines::common::read::read_vector;
use b_splines::common::vec_point3d::Point3D;
use b_splines::common::verification;
use b_splines::graphic::bspline;
use b_splines::graphic::plotting as plot;

fn main() {
    //* This is an example of the book "The NURBS Book" by Piegl and Tiller.

    // * Aqui comienza la lectura para el usuario

    println!("Bienvenido al programa de generación de B-Splines");
    println!("Seleccione la opción que desea realizar");
    println!("1. Generar una curva");
    println!("2. Generar una superficie");
    println!("Otro. Salir");
    print!("Opción: ");
    let opcion: i8 = read::read();

    match opcion {
        1 => curve(),
        2 => surface_example(),
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

fn surface() {
    // Lectura de nudos en U
    print!("Ingrese el numero de nudos en u: ");
    let n: i32 = read::read();
    let knots_u: Vec<f64> = read_vector(n);

    // Lectura de nudos en V
    print!("Ingrese el numero de nudos en v: ");
    let n: i32 = read::read();
    let knots_v: Vec<f64> = read_vector(n);

    // Lectura de puntos de control
    println!("Considere que los puntos de control se colocan en una matriz de n x m");
    print!("Ingrese el numero de columnas: ");
    let n: i32 = read::read();
    print!("Ingrese el numero de filas: ");
    let m: i32 = read::read();
    let points: Vec<Vec<Point3D>> = read::read_matrix(n, m);

    // Lectura del grado en U
    print!("Ingrese el grado en u: ");
    let p: i32 = read::read();

    // Lectura del grado en V
    print!("Ingrese el grado en v: ");
    let q: i32 = read::read();

    // Lectura de saltos en la superficie, define la resolucion de la superficie
    let u: f64 = 0.01;
    let v: f64 = 0.01;

    // Generacion de la superficie
    let bspline: Vec<Point3D> = bspline::bspline_surface(&knots_u, &knots_v, &points, u, v, p, q);

    print!("Ingrese el nombre del archivo para guardar el resultado (.png): ");
    let mut name: String = read::read();
    name = verification::nombre_con_extension(&name, ".png");

    plot::plot_surface(&name, &points, &bspline);
}

fn surface_example() {
    // * This is an example of the book "The NURBS Book" by Piegl and Tiller.

    // * Knots in U
    let knots_u: Vec<f64> = vec![0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0];

    // * Knots in V
    let knots_v: Vec<f64> = vec![0.0, 0.0, 0.0, 0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.0, 1.0, 1.0];

    // * Control points
    let points: Vec<Vec<Point3D>> = vec![
        vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(0.0, 2.0, 0.0),
            Point3D::new(0.0, 3.0, 0.0),
            Point3D::new(0.0, 4.0, 0.0),
            Point3D::new(0.0, 5.0, 0.0),
            Point3D::new(0.0, 6.0, 0.0),
            Point3D::new(0.0, 7.0, 0.0),

        ],
        vec![
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(1.0, 1.0, 0.0),
            Point3D::new(1.0, 2.0, 0.0),
            Point3D::new(1.0, 3.0, 0.0),
            Point3D::new(1.0, 4.0, 0.0),
            Point3D::new(1.0, 5.0, 0.0),
            Point3D::new(1.0, 6.0, 0.0),
            Point3D::new(1.0, 7.0, 0.0),
        ],
        vec![
            Point3D::new(2.0, 0.0, 0.0),
            Point3D::new(2.0, 1.0, 0.0),
            Point3D::new(2.0, 2.0, 0.0),
            Point3D::new(2.0, 3.0, 0.0),
            Point3D::new(2.0, 4.0, 0.0),
            Point3D::new(2.0, 5.0, 0.0),
            Point3D::new(2.0, 6.0, 0.0),
            Point3D::new(2.0, 7.0, 0.0),
        ],
        vec![
            Point3D::new(3.0, 0.0, 0.0),
            Point3D::new(3.0, 1.0, 0.0),
            Point3D::new(3.0, 2.0, 0.0),
            Point3D::new(3.0, 3.0, 0.0),
            Point3D::new(3.0, 4.0, 0.0),
            Point3D::new(3.0, 5.0, 0.0),
            Point3D::new(3.0, 6.0, 0.0),
            Point3D::new(3.0, 7.0, 0.0),
        ],
        vec![
            Point3D::new(4.0, 0.0, 0.0),
            Point3D::new(4.0, 1.0, 0.0),
            Point3D::new(4.0, 2.0, 0.0),
            Point3D::new(4.0, 3.0, 0.0),
            Point3D::new(4.0, 4.0, 0.0),
            Point3D::new(4.0, 5.0, 0.0),
            Point3D::new(4.0, 6.0, 0.0),
            Point3D::new(4.0, 7.0, 0.0),
        ],
        vec![
            Point3D::new(5.0, 0.0, 0.0),
            Point3D::new(5.0, 1.0, 0.0),
            Point3D::new(5.0, 2.0, 0.0),
            Point3D::new(5.0, 3.0, 0.0),
            Point3D::new(5.0, 4.0, 0.0),
            Point3D::new(5.0, 5.0, 0.0),
            Point3D::new(5.0, 6.0, 0.0),
            Point3D::new(5.0, 7.0, 0.0),
        ],
    ];

    // * Degree in U
    let p: i32 = 2;

    // * Degree in V
    let q: i32 = 3;

    // * Steps in the surface, defines the resolution of the surface
    let u: f64 = 0.01;
    let v: f64 = 0.01;

    // * Generation of the surface
    let bspline: Vec<Point3D> = bspline::bspline_surface(&knots_u, &knots_v, &points, u, v, p, q);

    // * Drawing of the surface
    plot::plot_surface("surface.png", &points, &bspline);
}
