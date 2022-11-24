use std::vec::Vec;
use b_spline::utils::stdiox as io;
use b_spline::geometry::points::Point;
use b_spline::geometry::vec_points::{self, Transform};
use b_spline::geometry::plot;
use b_spline::utils::math::{Float};

fn main() {
    
    let mut knot_vec:Vec<f64> = Vec::new();
    let mut ctrl_points = vec_points::new();

    //1
/*ctrl_points.push(Point::new(2.0,1.0));
ctrl_points.push(Point::new(2.0,3.0));
ctrl_points.push(Point::new(6.0,3.0));
ctrl_points.push(Point::new(6.0,9.0));
ctrl_points.push(Point::new(2.0,8.0));
ctrl_points.push(Point::new(6.0,14.0));
ctrl_points.push(Point::new(9.0,14.0));
ctrl_points.push(Point::new(9.0,3.0));
ctrl_points.push(Point::new(13.0,3.0));
ctrl_points.push(Point::new(13.0,1.0));
ctrl_points.push(Point::new(2.0,1.0));*/

//2
/*ctrl_points.push(Point::new(2.0,1.0));
ctrl_points.push(Point::new(2.0,3.0));
ctrl_points.push(Point::new(8.0,9.0));
ctrl_points.push(Point::new(6.0,12.0));
ctrl_points.push(Point::new(4.0,9.0));
ctrl_points.push(Point::new(2.0,10.0));
ctrl_points.push(Point::new(2.0,13.0));
ctrl_points.push(Point::new(5.0,14.0));
ctrl_points.push(Point::new(9.0,14.0));
ctrl_points.push(Point::new(11.0,9.0));
ctrl_points.push(Point::new(6.0,3.0));
ctrl_points.push(Point::new(13.0,1.0));
ctrl_points.push(Point::new(2.0,1.0));*/

//4
/*ctrl_points.push(Point::new(2.0,7.0));
ctrl_points.push(Point::new(2.0,12.0));
ctrl_points.push(Point::new(4.0,12.0));
ctrl_points.push(Point::new(4.0,10.0));
ctrl_points.push(Point::new(8.0,10.0));
ctrl_points.push(Point::new(8.0,13.0));
ctrl_points.push(Point::new(10.0,13.0));
ctrl_points.push(Point::new(10.0,1.0));
ctrl_points.push(Point::new(8.0,1.0));
ctrl_points.push(Point::new(8.0,8.0));
ctrl_points.push(Point::new(4.0,8.0));
ctrl_points.push(Point::new(4.0,6.0));
ctrl_points.push(Point::new(2.0,7.0));*/

   //3
   /* ctrl_points.push(Point::new(1.0,2.0));
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
    ctrl_points.push(Point::new(1.0,2.0));*/

/*A
ctrl_points.push(Point::new(1.0,1.0));
ctrl_points.push(Point::new(2.0,7.0));
ctrl_points.push(Point::new(4.0,9.0));
ctrl_points.push(Point::new(6.0,7.0));
ctrl_points.push(Point::new(7.0,1.0));
ctrl_points.push(Point::new(6.0,1.0));
ctrl_points.push(Point::new(5.0,5.0));
ctrl_points.push(Point::new(4.0,6.0));
ctrl_points.push(Point::new(3.0,5.0));
ctrl_points.push(Point::new(4.0,5.0));
ctrl_points.push(Point::new(2.0,1.0));
ctrl_points.push(Point::new(1.0,1.0));
     */ 

//B
/*ctrl_points.push(Point::new(2.0,1.0));
ctrl_points.push(Point::new(1.0,6.0));
ctrl_points.push(Point::new(2.0,10.0));
ctrl_points.push(Point::new(5.0,11.0));
ctrl_points.push(Point::new(6.0,9.0));
ctrl_points.push(Point::new(4.0,7.0));
ctrl_points.push(Point::new(4.0,9.0));
ctrl_points.push(Point::new(3.0,8.0));
ctrl_points.push(Point::new(6.0,6.0));
ctrl_points.push(Point::new(5.0,3.0));
ctrl_points.push(Point::new(4.0,1.0));
ctrl_points.push(Point::new(4.0,4.0));
ctrl_points.push(Point::new(3.0,5.0));
ctrl_points.push(Point::new(3.0,1.0));
ctrl_points.push(Point::new(2.0,1.0));*/

// C
/*ctrl_points.push(Point::new(2.0,1.0));
ctrl_points.push(Point::new(1.0,5.0));
ctrl_points.push(Point::new(3.0,9.0));
ctrl_points.push(Point::new(6.0,10.0));
ctrl_points.push(Point::new(9.0,8.0));
ctrl_points.push(Point::new(7.0,7.0));
ctrl_points.push(Point::new(5.0,8.0));
ctrl_points.push(Point::new(4.0,5.0));
ctrl_points.push(Point::new(5.0,2.0));
ctrl_points.push(Point::new(8.0,3.0));
ctrl_points.push(Point::new(9.0,1.0));
ctrl_points.push(Point::new(5.0,0.0));
ctrl_points.push(Point::new(2.0,1.0));*/

//D
/*ctrl_points.push(Point::new(2.0,0.0));
ctrl_points.push(Point::new(2.0,6.0));
ctrl_points.push(Point::new(2.0,11.0));
ctrl_points.push(Point::new(7.0,11.0));
ctrl_points.push(Point::new(10.0,9.0));
ctrl_points.push(Point::new(11.0,6.0));
ctrl_points.push(Point::new(10.0,0.0));
ctrl_points.push(Point::new(5.0,8.0));
ctrl_points.push(Point::new(5.0,3.0));
ctrl_points.push(Point::new(9.0,3.0));
ctrl_points.push(Point::new(10.0,0.0));
ctrl_points.push(Point::new(5.0,0.0));
ctrl_points.push(Point::new(2.0,0.0));*/

//E
/*ctrl_points.push(Point::new(3.0,0.0));
ctrl_points.push(Point::new(2.0,5.0));
ctrl_points.push(Point::new(3.0,9.0));
ctrl_points.push(Point::new(9.0,10.0));
ctrl_points.push(Point::new(10.0,8.0));
ctrl_points.push(Point::new(7.0,7.0));
ctrl_points.push(Point::new(5.0,6.0));
ctrl_points.push(Point::new(8.0,6.0));
ctrl_points.push(Point::new(9.0,4.0));
ctrl_points.push(Point::new(5.0,3.0));
ctrl_points.push(Point::new(10.0,2.0));
ctrl_points.push(Point::new(10.0,0.0));
ctrl_points.push(Point::new(3.0,0.0));*/

//F
/*ctrl_points.push(Point::new(3.0,0.0));
ctrl_points.push(Point::new(2.0,5.0));
ctrl_points.push(Point::new(3.0,9.0));
ctrl_points.push(Point::new(9.0,10.0));
ctrl_points.push(Point::new(10.0,8.0));
ctrl_points.push(Point::new(7.0,7.0));
ctrl_points.push(Point::new(5.0,6.0));
ctrl_points.push(Point::new(8.0,6.0));
ctrl_points.push(Point::new(9.0,4.0));
ctrl_points.push(Point::new(5.0,4.0));
ctrl_points.push(Point::new(5.0,2.0));
ctrl_points.push(Point::new(5.0,0.0));
ctrl_points.push(Point::new(3.0,0.0));*/

//G
/*ctrl_points.push(Point::new(3.0,0.0));
ctrl_points.push(Point::new(2.0,5.0));
ctrl_points.push(Point::new(3.0,9.0));
ctrl_points.push(Point::new(9.0,10.0));
ctrl_points.push(Point::new(10.0,8.0));
ctrl_points.push(Point::new(5.0,7.0));
ctrl_points.push(Point::new(4.0,4.0));
ctrl_points.push(Point::new(7.0,2.0));
ctrl_points.push(Point::new(7.0,5.0));
ctrl_points.push(Point::new(10.0,4.0));
ctrl_points.push(Point::new(10.0,1.0));
ctrl_points.push(Point::new(5.0,0.0));
ctrl_points.push(Point::new(3.0,0.0));*/

//H
/*ctrl_points.push(Point::new(3.0,0.0));
ctrl_points.push(Point::new(2.0,5.0));
ctrl_points.push(Point::new(2.0,10.0));
ctrl_points.push(Point::new(5.0,10.0));
ctrl_points.push(Point::new(5.0,6.0));
ctrl_points.push(Point::new(8.0,5.0));
ctrl_points.push(Point::new(11.0,4.0));
ctrl_points.push(Point::new(13.0,0.0));
ctrl_points.push(Point::new(10.0,0.0));
ctrl_points.push(Point::new(9.0,3.0));
ctrl_points.push(Point::new(4.0,4.0));
ctrl_points.push(Point::new(6.0,0.0));
ctrl_points.push(Point::new(3.0,0.0));*/

//I
/*ctrl_points.push(Point::new(0.0,0.0));
ctrl_points.push(Point::new(3.0,2.0));
ctrl_points.push(Point::new(4.0,5.0));
ctrl_points.push(Point::new(3.0,8.0));
ctrl_points.push(Point::new(2.0,10.0));
ctrl_points.push(Point::new(5.0,12.0));
ctrl_points.push(Point::new(8.0,10.0));
ctrl_points.push(Point::new(7.0,8.0));
ctrl_points.push(Point::new(6.0,5.0));
ctrl_points.push(Point::new(7.0,2.0));
ctrl_points.push(Point::new(10.0,0.0));
ctrl_points.push(Point::new(5.0,0.0));
ctrl_points.push(Point::new(0.0,0.0));*/

//J
/*ctrl_points.push(Point::new(3.0,3.0));
ctrl_points.push(Point::new(4.0,6.0));
ctrl_points.push(Point::new(6.0,3.0));
ctrl_points.push(Point::new(7.0,5.0));
ctrl_points.push(Point::new(5.0,10.0));
ctrl_points.push(Point::new(2.0,12.0));
ctrl_points.push(Point::new(7.0,12.0));
ctrl_points.push(Point::new(9.0,6.0));
ctrl_points.push(Point::new(10.0,3.0));
ctrl_points.push(Point::new(10.0,1.0));
ctrl_points.push(Point::new(8.0,0.0));
ctrl_points.push(Point::new(5.0,0.0));
ctrl_points.push(Point::new(3.0,3.0));*/

//K
/*ctrl_points.push(Point::new(2.0,0.0));
ctrl_points.push(Point::new(1.0,7.0));
ctrl_points.push(Point::new(1.0,12.0));
ctrl_points.push(Point::new(4.0,11.0));
ctrl_points.push(Point::new(4.0,8.0));
ctrl_points.push(Point::new(10.0,11.0));
ctrl_points.push(Point::new(10.0,9.0));
ctrl_points.push(Point::new(5.0,6.0));
ctrl_points.push(Point::new(12.0,0.0));
ctrl_points.push(Point::new(9.0,0.0));
ctrl_points.push(Point::new(3.0,5.0));
ctrl_points.push(Point::new(5.0,0.0));
ctrl_points.push(Point::new(2.0,0.0));*/

//L
/*ctrl_points.push(Point::new(1.0,1.0));
ctrl_points.push(Point::new(1.0,12.0));
ctrl_points.push(Point::new(4.0,12.0));
ctrl_points.push(Point::new(3.0,8.0));
ctrl_points.push(Point::new(3.0,4.0));
ctrl_points.push(Point::new(5.0,3.0));
ctrl_points.push(Point::new(8.0,3.0));
ctrl_points.push(Point::new(10.0,3.0));
ctrl_points.push(Point::new(12.0,1.0));
ctrl_points.push(Point::new(9.0,1.0));
ctrl_points.push(Point::new(6.0,1.0));
ctrl_points.push(Point::new(3.0,1.0));
ctrl_points.push(Point::new(1.0,1.0));*/

//M
/*ctrl_points.push(Point::new(1.0,1.0));
ctrl_points.push(Point::new(2.0,12.0));
ctrl_points.push(Point::new(4.0,10.0));
ctrl_points.push(Point::new(6.0,6.0));
ctrl_points.push(Point::new(8.0,10.0));
ctrl_points.push(Point::new(10.0,12.0));
ctrl_points.push(Point::new(12.0,1.0));
ctrl_points.push(Point::new(10.0,1.0));
ctrl_points.push(Point::new(8.0,6.0));
ctrl_points.push(Point::new(6.0,3.0));
ctrl_points.push(Point::new(4.0,6.0));
ctrl_points.push(Point::new(3.0,1.0));
ctrl_points.push(Point::new(1.0,1.0));*/

//N
/*ctrl_points.push(Point::new(1.0,0.0));
ctrl_points.push(Point::new(2.0,12.0));
ctrl_points.push(Point::new(5.0,12.0));
ctrl_points.push(Point::new(6.0,6.0));
ctrl_points.push(Point::new(8.0,3.0));
ctrl_points.push(Point::new(9.0,12.0));
ctrl_points.push(Point::new(12.0,13.0));
ctrl_points.push(Point::new(11.0,1.0));
ctrl_points.push(Point::new(8.0,1.0));
ctrl_points.push(Point::new(6.0,3.0));
ctrl_points.push(Point::new(4.0,6.0));
ctrl_points.push(Point::new(4.0,1.0));
ctrl_points.push(Point::new(1.0,0.0));*/

//O
/*ctrl_points.push(Point::new(5.0,0.0));
ctrl_points.push(Point::new(1.0,3.0));
ctrl_points.push(Point::new(1.0,7.0));
ctrl_points.push(Point::new(3.0,11.0));
ctrl_points.push(Point::new(6.0,12.0));
ctrl_points.push(Point::new(9.0,11.0));
ctrl_points.push(Point::new(11.0,7.0));
ctrl_points.push(Point::new(11.0,3.0));
ctrl_points.push(Point::new(8.0,0.0));
ctrl_points.push(Point::new(4.0,0.0));
ctrl_points.push(Point::new(9.0,4.0));
ctrl_points.push(Point::new(5.0,7.0));
ctrl_points.push(Point::new(5.0,0.0));*/

//P
/*ctrl_points.push(Point::new(2.0,0.0));
ctrl_points.push(Point::new(2.0,13.0));
ctrl_points.push(Point::new(4.0,13.0));
ctrl_points.push(Point::new(4.0,12.0));
ctrl_points.push(Point::new(8.0,13.0));
ctrl_points.push(Point::new(11.0,11.0));
ctrl_points.push(Point::new(11.0,7.0));
ctrl_points.push(Point::new(10.0,4.0));
ctrl_points.push(Point::new(5.0,4.0));
ctrl_points.push(Point::new(4.0,7.0));
ctrl_points.push(Point::new(8.0,7.0));
ctrl_points.push(Point::new(5.0,10.0));
ctrl_points.push(Point::new(2.0,0.0));*/

//Q
 /*ctrl_points.push(Point::new(5.0,1.0));
ctrl_points.push(Point::new(2.0,3.0));
ctrl_points.push(Point::new(2.0,7.0));
ctrl_points.push(Point::new(4.0,11.0));
ctrl_points.push(Point::new(8.0,12.0));
ctrl_points.push(Point::new(13.0,9.0));
ctrl_points.push(Point::new(11.0,2.0));
ctrl_points.push(Point::new(7.0,8.0));
ctrl_points.push(Point::new(7.0,5.0));
ctrl_points.push(Point::new(15.0,4.0));
ctrl_points.push(Point::new(12.0,0.0));
ctrl_points.push(Point::new(11.0,3.0));
ctrl_points.push(Point::new(5.0,1.0));*/

//R
/*ctrl_points.push(Point::new(2.0,1.0));
ctrl_points.push(Point::new(2.0,7.0));
ctrl_points.push(Point::new(2.0,13.0));
ctrl_points.push(Point::new(4.0,13.0));
ctrl_points.push(Point::new(6.0,14.0));
ctrl_points.push(Point::new(8.0,13.0));
ctrl_points.push(Point::new(10.0,10.0));
ctrl_points.push(Point::new(8.0,8.0));
ctrl_points.push(Point::new(5.0,9.0));
ctrl_points.push(Point::new(12.0,1.0));
ctrl_points.push(Point::new(9.0,1.0));
ctrl_points.push(Point::new(4.0,6.0));
ctrl_points.push(Point::new(2.0,1.0));*/

//S
/*ctrl_points.push(Point::new(2.0,1.0));
ctrl_points.push(Point::new(0.0,5.0));
ctrl_points.push(Point::new(2.0,6.0));
ctrl_points.push(Point::new(5.0,3.0));
ctrl_points.push(Point::new(2.0,11.0));
ctrl_points.push(Point::new(6.0,14.0));
ctrl_points.push(Point::new(11.0,12.0));
ctrl_points.push(Point::new(10.0,9.0));
ctrl_points.push(Point::new(5.0,9.0));
ctrl_points.push(Point::new(11.0,4.0));
ctrl_points.push(Point::new(9.0,1.0));
ctrl_points.push(Point::new(4.0,1.0));
ctrl_points.push(Point::new(2.0,1.0));*/

//T
/*ctrl_points.push(Point::new(6.0,2.0));
ctrl_points.push(Point::new(6.0,5.0));
ctrl_points.push(Point::new(6.0,11.0));
ctrl_points.push(Point::new(2.0,12.0));
ctrl_points.push(Point::new(2.0,14.0));
ctrl_points.push(Point::new(7.0,14.0));
ctrl_points.push(Point::new(12.0,14.0));
ctrl_points.push(Point::new(12.0,12.0));
ctrl_points.push(Point::new(8.0,11.0));
ctrl_points.push(Point::new(8.0,5.0));
ctrl_points.push(Point::new(8.0,2.0));
ctrl_points.push(Point::new(7.0,2.0));
ctrl_points.push(Point::new(6.0,2.0));*/

//U
/*ctrl_points.push(Point::new(5.0,3.0));
ctrl_points.push(Point::new(3.0,7.0));
ctrl_points.push(Point::new(2.0,12.0));
ctrl_points.push(Point::new(5.0,12.0));
ctrl_points.push(Point::new(6.0,7.0));
ctrl_points.push(Point::new(7.0,6.0));
ctrl_points.push(Point::new(8.0,7.0));
ctrl_points.push(Point::new(9.0,12.0));
ctrl_points.push(Point::new(12.0,12.0));
ctrl_points.push(Point::new(11.0,7.0));
ctrl_points.push(Point::new(9.0,3.0));
ctrl_points.push(Point::new(7.0,2.0));
ctrl_points.push(Point::new(5.0,3.0));*/

//V
/*ctrl_points.push(Point::new(5.0,3.0));
ctrl_points.push(Point::new(3.0,7.0));
ctrl_points.push(Point::new(1.0,13.0));
ctrl_points.push(Point::new(4.0,12.0));
ctrl_points.push(Point::new(5.0,9.0));
ctrl_points.push(Point::new(7.0,5.0));
ctrl_points.push(Point::new(9.0,9.0));
ctrl_points.push(Point::new(10.0,12.0));
ctrl_points.push(Point::new(13.0,13.0));
ctrl_points.push(Point::new(11.0,7.0));
ctrl_points.push(Point::new(9.0,3.0));
ctrl_points.push(Point::new(7.0,0.0));
ctrl_points.push(Point::new(5.0,3.0));*/

//W
/*ctrl_points.push(Point::new(4.0,1.0));
ctrl_points.push(Point::new(2.0,5.0));
ctrl_points.push(Point::new(1.0,13.0));
ctrl_points.push(Point::new(4.0,12.0));
ctrl_points.push(Point::new(5.0,9.0));
ctrl_points.push(Point::new(7.0,7.0));
ctrl_points.push(Point::new(9.0,9.0));
ctrl_points.push(Point::new(10.0,12.0));
ctrl_points.push(Point::new(13.0,13.0));
ctrl_points.push(Point::new(12.0,5.0));
ctrl_points.push(Point::new(10.0,1.0));
ctrl_points.push(Point::new(7.0,5.0));
ctrl_points.push(Point::new(4.0,1.0));*/

//X
/*ctrl_points.push(Point::new(4.0,4.0));
ctrl_points.push(Point::new(2.0,5.0));
ctrl_points.push(Point::new(6.0,10.0));
ctrl_points.push(Point::new(3.0,12.0));
ctrl_points.push(Point::new(2.0,15.0));
ctrl_points.push(Point::new(7.0,12.0));
ctrl_points.push(Point::new(12.0,15.0));
ctrl_points.push(Point::new(11.0,12.0));
ctrl_points.push(Point::new(8.0,10.0));
ctrl_points.push(Point::new(12.0,5.0));
ctrl_points.push(Point::new(10.0,4.0));
ctrl_points.push(Point::new(7.0,8.0));
ctrl_points.push(Point::new(4.0,4.0));*/

//Y
/*ctrl_points.push(Point::new(6.0,2.0));
ctrl_points.push(Point::new(6.0,5.0));
ctrl_points.push(Point::new(6.0,10.0));
ctrl_points.push(Point::new(3.0,12.0));
ctrl_points.push(Point::new(2.0,15.0));
ctrl_points.push(Point::new(7.0,12.0));
ctrl_points.push(Point::new(12.0,15.0));
ctrl_points.push(Point::new(11.0,12.0));
ctrl_points.push(Point::new(8.0,10.0));
ctrl_points.push(Point::new(8.0,5.0));
ctrl_points.push(Point::new(8.0,2.0));
ctrl_points.push(Point::new(7.0,2.0));
ctrl_points.push(Point::new(6.0,2.0));*/

//Z
/*ctrl_points.push(Point::new(2.0,1.0));
ctrl_points.push(Point::new(2.0,5.0));
ctrl_points.push(Point::new(8.0,11.0));
ctrl_points.push(Point::new(2.0,8.0));
ctrl_points.push(Point::new(2.0,12.0));
ctrl_points.push(Point::new(11.0,13.0));
ctrl_points.push(Point::new(11.0,11.0));
ctrl_points.push(Point::new(8.0,7.0));
ctrl_points.push(Point::new(5.0,4.0));
ctrl_points.push(Point::new(14.0,6.0));
ctrl_points.push(Point::new(13.0,3.0));
ctrl_points.push(Point::new(13.0,1.0));
ctrl_points.push(Point::new(2.0,1.0));*/






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
    println!("\nCURVE :\n {}\n", curve.to_string());
    let fname = io::read_string("Nombre de archivo: ");
    plot::plot_graph("curve", &fname, &ctrl_points, &curve);
    plot::plot_pollygon(&fname, &ctrl_points, &curve);
    print!("\tFINISH\n\n#\n\n");
    



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
    n.push(1.0);
    let left = |j:i32| -> f64 {u - knot_vec[(i + 1 - j) as usize]};
    let right = |j:i32| -> f64 {knot_vec[(i + j) as usize] - u};
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



