use b_splines::common::point::Point;
use b_splines::graphic::curves as curves;
use b_splines::graphic::plotting as plot;

fn main() {
    // Knot vector, must be always increasing
    let knots:Vec<f64> = vec![0.0,0.0,0.0,0.125,0.25,0.375,0.5,0.625,0.75,0.875,1.0,1.0,1.0];

    // Control points
    let points:Vec<Point> = vec![
        Point::new(0.0, 5.0),
        Point::new(1.0, 7.0),
        Point::new(2.0, 5.0),
        Point::new(4.0, 10.0),
        Point::new(8.0, 9.0),
        Point::new(10.0, 0.0),
        Point::new(14.0, 5.0),
        Point::new(15.0, 3.0),
        Point::new(19.0, 8.0),
        Point::new(20.0, 7.0)
    ];

    // Degree
    let p:i32 = 2;

    // Generate the curve
    let bspline: Vec<Point> = curves::bspline(&knots, &points, p, 0.0001);

    let mut name:String = "name".to_owned();
    name.push_str(".png");
    // Draw the curve
    plot::plot_curve("Curva",&name, &points, &bspline, 1200,800);

}
