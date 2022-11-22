use std::vec::Vec;
use b_spline::utils::stdiox as io;
use b_spline::geometry::points::Point;
use b_spline::geometry::vec_points::{self, Transform}; //, new
use b_spline::geometry::plot;
use b_spline::utils::math::{Float}; //self, 

fn main() {//                                                 u=2.5
    
    
    let mut knot_vec:Vec<f64> = Vec::new();

    let mut ctrl_points = vec_points::new();

    
    print!("#\n\n\tB-SPLINE\n\n#\n\n");
   
    ctrl_points.push(Point::new(1.0,2.0));
    ctrl_points.push(Point::new(2.0,3.0));
    ctrl_points.push(Point::new(5.0,4.0));
    ctrl_points.push(Point::new(3.0,5.0));
    ctrl_points.push(Point::new(5.0,6.0));
    ctrl_points.push(Point::new(4.0,8.0));
    ctrl_points.push(Point::new(1.0,7.0));
    ctrl_points.push(Point::new(2.0,10.0));
    ctrl_points.push(Point::new(5.0,11.0));
    ctrl_points.push(Point::new(7.0,10.0));
    ctrl_points.push(Point::new(8.0,8.0));
    ctrl_points.push(Point::new(8.0,6.0));
    ctrl_points.push(Point::new(7.0,5.0));
    ctrl_points.push(Point::new(9.0,4.0));
    ctrl_points.push(Point::new(10.0,2.0));
    ctrl_points.push(Point::new(9.0,1.0));
    ctrl_points.push(Point::new(6.0,0.0));
    ctrl_points.push(Point::new(2.0,1.0));
    ctrl_points.push(Point::new(1.0,2.0));

    let no_pts:i32 = ctrl_points.len() as i32;
    let p:i32 = 2;
    let n:i32 = no_pts + p + 1;

    for k in 0..=p {
        println!("u[{}] = 0.0",k);
        knot_vec.push(0.0);
    }

    let denom:f64 = (no_pts - p) as f64;
    for k in p + 1..n - p - 1 {
        
        let num:f64 = (k - p) as f64 / denom;
        println!("u[{}] = {}", k, num);
        knot_vec.push(num);
    }

    for k in n - p - 1..n {
        println!("u[{}] = {}",k, 1.0.precision(2));
        knot_vec.push(1.0);
    }


    let mut curve: Vec<Point> = vec_points::new();

    let mut u:f64 = 0.0;
    while u <= knot_vec[knot_vec.len() - 1] {
        
        curve.push(curve_point(p, u, &knot_vec, &ctrl_points));
        u = (u + 0.01).precision(2);
    }

    println!("\nCURVA :\n {}\n", curve.to_string());
    let fname = io::read_string("Escriba el nombre del archivo: ");

    //-------------------------------
    plot::plot_graph("Curva B-Spline", &fname, &ctrl_points, &curve);
    

    print!("\tTerminado!!\n\n#\n\n");
    
}

fn find_span(p:i32, u:f64, knot_vec:&Vec<f64>) -> i32 {
    let m:i32 = (knot_vec.len() - 1) as i32;
    
    if u.precision(2) == knot_vec[(m - p + 1) as usize].precision(2) {
        return m - p - 1;
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

    
    for k in 0..=p {
        curve_point = Point::sum(
            &curve_point,
            &Point::mul(n[k as usize], 
                &ctrl_points[(i - p + k) as usize]) );
    }

    return curve_point;
}



