use bezier_curves::geometry::points::Point;
use bezier_curves::geometry::vec_points;
use bezier_curves::geometry::vec_points::{Transform};
use bezier_curves::utils::stdiox as io;
use bezier_curves::graphing::bezier;
use bezier_curves::geometry::plot;

fn main() {
    let mut ctrl_points = vec_points::new();

    println!("----------------------------------------------------");
    println!("        POLINOMIO DE BERNSTEIN");
    println!("----------------------------------------------------");
    
    let n:i32 = io::read_i32("Escriba el numero de puntos de control: ");

    println!("----------------------------------------------------");
    
    // ------------< LECTURA DE PUNTOS >----------------
    
    let mut x:f64;
    let mut y:f64;

    for i in 0..n {
        println!("\n\t(P-{})", i + 1);
        
        x = io::read_f64("\t\tx = ");
        y = io::read_f64("\t\ty = ");

        let point = Point::new(x, y);

        ctrl_points.push(point);
    }

    println!("\n\tPUNTOS DE CONTROL:\n{}", ctrl_points.to_string());
    // -------------------------------------------------

    // ----------< CALCULO DE LA CURVA >----------------

    let curve = bezier::bez(100, &ctrl_points);

    println!("\n\tPUNTOS DE LA CURVA:\n{}", curve.to_string());

    //--------------------------------------------------

    // ----------< GRAFICA DE LA CURVA >----------------
    
    println!("----------------------------------------------------");

    let title = format!("Curva de Bezier - {} Puntos de Control", n);
    let fname = io::read_String("\tEscriba el nombre del archivo PNG (sin extension) : ");

    println!("\n\tGraficando resultado...");
    plot::plot_graph(&*title, &*format!("src/images/{}{}", fname, ".png"), &ctrl_points, &curve);
    //plot::plot_graph(&*title, "src/images/chart_test.png", ctrl_points, curve);

    println!("\tOperacion exitosa!!\n\n");
}
