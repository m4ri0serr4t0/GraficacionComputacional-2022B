use crate::geometry::points::Point;
use crate::geometry::vec_points;
use crate::graphing::bernstein;

/* a mayor num_points, mayor detalle en la curva */
pub fn bez(num_points:i32, ctrl_points:&Vec<Point>) -> Vec<Point> {
	let mut graph = vec_points::new(); // curva
    let mut pixel:Point;

	let n = ctrl_points.len() - 1; // se dan n + 1 puntos de control
    let inc:f64 = 1.0 / num_points as f64;
    let mut u:f64 = 0.0;
    let mut pol:f64;

	while u < 1.0 + inc { // construye la curva
        pixel = Point::new(0.0,0.0);

        for i in 0..=n { // calcula C(u) -> un punto en la curva

            // P(i)
            let point = ctrl_points.get(i).expect("");

            // B(i,n)
            pol = bernstein::bern(i.try_into().unwrap(), n.try_into().unwrap(), u); 

            // Z P(i) . B(i,n)
            pixel = Point::sum(&pixel, &Point::mul(pol, point));
        }

        graph.push(pixel);

        u = u + inc;
        //println!("u = {}", u);
    }

    return graph;
}