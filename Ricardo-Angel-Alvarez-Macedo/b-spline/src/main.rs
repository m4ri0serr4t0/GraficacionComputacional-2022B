use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::process;
use std::vec::Vec;

use b_spline::geometry::plot;
use b_spline::geometry::points::Point;
use b_spline::geometry::vec_points::{self, new, Transform};
use b_spline::utils::math::{self, Float};

fn run(ctrl_points: &mut Vec<Point>,file_path:&str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let x: f64 = record[0].parse()?;
        let y: f64 = record[1].parse()?;
        println!("x: {:?}, y: {:?}", x, y);
        ctrl_points.push(Point::new(x, y));
    }
    Ok(())
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {//                                                 u=2.5
    let paths = fs::read_dir("points/").unwrap();

    for path in paths {
        let path_name=path.as_ref().unwrap().file_name();
        let mut path_name_re=path_name.to_str().unwrap().to_string();
        path_name_re="out/".to_string()+ &*path_name_re.replace(".csv", ".png").to_string();
        println!("Name: {:?}",path_name);
        println!("Name_Remp: {:?}",path_name_re);

        let mut knot_vec: Vec<f64> = Vec::new();

        let mut ctrl_points = new();

        println!("Name_Remp: {:?}",&*path.as_ref().unwrap().path().to_str().unwrap().to_string());
        if let Err(err) = run(&mut ctrl_points, &*path.unwrap().path().to_str().unwrap().to_string()) {
            println!("{}", err);
            process::exit(1);
        }

        let no_pts: i32 = ctrl_points.len() as i32;
        let p: i32 = 2;
        let n: i32 = no_pts + p + 1;

        for k in 0..=p {
            println!("u[{}] = 0.0", k);
            knot_vec.push(0.0);
        }

        let denom: f64 = (no_pts - p) as f64;
        for k in p + 1..n - p - 1 {
            let num: f64 = (k - p) as f64 / denom;
            println!("u[{}] = {}", k, num);
            knot_vec.push(num);
        }

        for k in n - p - 1..n {
            println!("u[{}] = {}", k, 1.0.precision(2));
            knot_vec.push(1.0);
        }


        let mut curve: Vec<Point> = new();

        let mut u: f64 = 0.0;
        while u <= knot_vec[knot_vec.len() - 1] {
            curve.push(curve_point(p, u, &knot_vec, &ctrl_points));
            u = (u + 0.01).precision(2);
        }

        //-------------------------------
        //plot::plot_graph("Curva B-Spline", &*path_name_re, &ctrl_points, &curve);
        plot::plot_pollygon(&*path_name_re, &ctrl_points, &curve);

        print!("\tListo!!\n");
    }
}

fn find_span(p: i32, u: f64, knot_vec: &Vec<f64>) -> i32 {
    let m: i32 = (knot_vec.len() - 1) as i32;

    if u.precision(2) == knot_vec[(m - p + 1) as usize].precision(2) {
        return m - p - 1;
    }

    let mut low = p;
    let mut high = m - p;
    let mut mid: i32 = (low + high) / 2;

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

fn basis_funs(i: i32, u: f64, p: i32, knot_vec: &Vec<f64>) -> Vec<f64> {
    let mut n: Vec<f64> = Vec::new();

    // N[0] = 1
    n.push(1.0);

    // Closures
    let left = |j: i32| -> f64 { u - knot_vec[(i + 1 - j) as usize] };
    let right = |j: i32| -> f64 { knot_vec[(i + j) as usize] - u };

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

fn curve_point(p: i32, u: f64, knot_vec: &Vec<f64>, ctrl_points: &Vec<Point>) -> Point {
    let i = find_span(p, u, &knot_vec);

    let n = basis_funs(i, u, p, &knot_vec);

    let mut curve_point: Point = Point::new(0.0, 0.0);

    //C(u) = P(i-p)N(i-p, p) + P(i-p+1)N(i-p+1, p) + P(i-p+2)N(i-p+2, p) + ...
    for k in 0..=p {
        curve_point = Point::sum(
            &curve_point,
            &Point::mul(n[k as usize],
                        &ctrl_points[(i - p + k) as usize]));
    }

    return curve_point;
}