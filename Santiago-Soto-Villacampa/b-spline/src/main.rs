use std::vec::Vec;
//use b_spline::utils::stdiox;
use b_spline::geometry::points::Point;
use b_spline::geometry::vec_points::{self, Transform};
use b_spline::geometry::plot;

/*

                    |
                    |
                    |
                    |                          . 12 Puntos de control = Polinomio en piezas de grado 12.
                    |                         /  Como P(1,3) = P(2,0) y P(2,3) = P(3,0), el grado real es solo 10.
                    |           i = 1        /       
                    |                       /
                    |              _..o P  /
                    |            _/     \ /
                    |          ./         .                                 
                    |         /            o. P(1,3) = P(2,0)    ___     i = 3
                    |        o P(1,1)      l \.                +    l+      
                    |       /              l   \             /      l  \
                    |      |               l    \  i = 2   +        l    .
                    |     |                l     \      ./          l      \
                    |    |                 l       . _--            l        +              m = 3 segmentos 
                    |   |                  l                        l         \             "i" va de 1 a m - 1 <-- recorre los segmentos C(u)
                    |   o P(1,0)           l                        l          \            u E [0,1]
    breakpoints ------> u0 = 0            u1                       u2          u3 = 1       u0 < u1 < u2 < u3
                    |_________________________________________________________________________________

                        P(j,i) -> j = segmento
                               -> i = punto (del segmento)
                        
                        u1 y u2 pueden ser discontinuos.
*/

fn main() {//                                                u=2.5
    //U = {0,0,0,1,2,3,4,4,5,5,5}       0    1    2    3    4    5    6    7    8    9    10
    //let knot_vec:Vec<f64> = Vec::from([0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0]);

    // Ejemplo: pag. 88
    //let knot_vec:Vec<f64> = Vec::from([0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0]);
    let knot_vec:Vec<f64> = Vec::from([0.0,0.0,0.0,0.125,0.25,0.375,0.5,0.625,0.75,0.875,1.0,1.0,1.0]);

    let mut ctrl_points = vec_points::new();

    ctrl_points.push(Point::new(0.0, 5.0));
    ctrl_points.push(Point::new(1.0, 7.0));
    ctrl_points.push(Point::new(2.0, 5.0));
    ctrl_points.push(Point::new(4.0, 10.0));
    ctrl_points.push(Point::new(8.0, 9.0));
    ctrl_points.push(Point::new(10.0, 0.0));
    ctrl_points.push(Point::new(14.0, 5.0));
    ctrl_points.push(Point::new(15.0, 3.0));
    ctrl_points.push(Point::new(19.0, 8.0));
    ctrl_points.push(Point::new(20.0, 7.0));

    //let p = (ctrl_points.len() - 1) as i32; 
    let p = 2;

    let mut curve: Vec<Point> = vec_points::new();

    let mut u:f64 = 0.0;
    while u <= knot_vec[knot_vec.len() - 1] {
        curve.push(curve_point(p, u, &knot_vec, &ctrl_points));
        u = u + 0.01;
    }

    println!("Curva : {}", curve.to_string());

    plot::plot_graph("Curva B-Spline", "B_Spline_04.png", &ctrl_points, &curve);

    
}

fn find_span(p:i32, u:f64, knot_vec:&Vec<f64>) -> i32 {
    let m:i32 = (knot_vec.len() - 1) as i32;
    
    if u == knot_vec[(m - p + 1) as usize] {
        return m - p + 1;
    }

    let mut low = p;        
    let mut high= m - p;    
    let mut mid:i32 = (low + high) / 2; 

    while u < knot_vec[mid as usize] || u >= knot_vec[(mid + 1) as usize] {

        if u < knot_vec[mid as usize] {
            high = mid;
        } else {
            low = mid;
        }
        mid = (low + high) / 2;
    }

    return mid;
}

fn basis_funs(i:i32, u:f64, p:i32, knot_vec:&Vec<f64>) -> Vec<f64>{
    let mut n:Vec<f64> = Vec::new();

    // N[0] = 1
    n.push(1.0);

    // Closures
    let left = |j:i32| -> f64 {u - knot_vec[(i + 1 - j) as usize]};
    let right = |j:i32| -> f64 {knot_vec[(i + j) as usize] - u};

    // j -> N(i,1) ... N(i, p)
    for j in 1..=p {
        
        let mut saved = 0.0;
        for r in 0..j {
            let temp = n[r as usize] / (right(r + 1) + left(j - r));

            n[r as usize] = saved + right(r + 1) * temp;

            saved = left(j - r) * temp;
        }
        n.push(saved);
    }

    return n;
}

fn curve_point(p:i32, u:f64, knot_vec:&Vec<f64>, ctrl_points:&Vec<Point>) -> Point {
    let i = find_span(p, u, &knot_vec);
    let n = basis_funs(i, u, p, &knot_vec);

    let mut curve_point:Point = Point::new(0.0, 0.0);

    //C(u) = P(i-p)N(i-p, p) + P(i-p+1)N(i-p+1, p) + P(i-p+2)N(i-p+2, p) + ...
    for k in 0..=p {
        curve_point = Point::sum(
            &curve_point,
            &Point::mul(n[k as usize], 
                &ctrl_points[(i - p + k) as usize]) );
    }

    return curve_point;
}



