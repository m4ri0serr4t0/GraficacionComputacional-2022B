use crate::common::point::Point;
use crate::common::vec_point;
use crate::common::point3d::Point3D;
use crate::common::vec_point3d;

pub fn bspline(knots:&Vec<f64>, points:&Vec<Point>, p:i32, step:f64) -> Vec<Point> {
    let n:i32 = (points.len() - 1) as i32;
    let mut u:f64 = 0.0;
    let mut bspline: Vec<Point> = vec_point::new();
    while u <= 1.0 {
        bspline.push(curve_point(n, p, u, &knots, &points));
        u = u + step;
    }
    return bspline;
}
    

fn curve_point(n:i32,p:i32,u:f64,knots:&Vec<f64>,points:&Vec<Point>) -> Point {
    let span:i32 = find_span(n,p, u, &knots);
    let basisfuns:Vec<f64> = basis_funs(span, u, p, &knots);

    let mut curve_point:Point = Point::new(0.0, 0.0);

    for j in 0..=p {
        curve_point = curve_point + points[(span - p + j) as usize] * basisfuns[j as usize];
    }

    return curve_point;
}

pub fn bspline_surface(knots_u:&Vec<f64>, knots_v:&Vec<f64>, points:&Vec<Vec<Point3D>>, mut u:f64, mut v:f64, p:i32, q:i32) -> Vec<Point3D> {
    let n:i32 = (points.len() - 1)  as i32;
    let m:i32 = (points[0].len() - 1) as i32;
    let mut surface:Vec<Point3D> = vec_point3d::new();
    while u <= 1.0 {
        while v <= 1.0 {
            surface.push(surface_point(n, p,&knots_u, m, q, &knots_v, &points, u, v));
            v = v + 0.01;
        }
        u = u + 0.01;
    }
    return surface;
}

fn surface_point(n:i32, p:i32, knots_u:&Vec<f64>, m:i32, q:i32, knots_v:&Vec<f64>, points:&Vec<Vec<Point3D>>, u:f64, v:f64) -> Point3D {
    let uspan = find_span(n, p, u, &knots_u);
    let vspan = find_span(m, q, v, &knots_v);
    let nu = basis_funs(uspan, u, p, &knots_u);
    let nv = basis_funs(vspan, v, q, &knots_v);
    let mut temp = vec![Point3D::new(0.0, 0.0, 0.0); q as usize];
     for l in 0..q {
        for k in 0..q {
            temp[l as usize] = temp[l as usize] + points[(uspan - p + k) as usize][(vspan - q + l) as usize] * nu[k as usize]; 
        }
    }
    let mut sum:Point3D = Point3D::new(0.0, 0.0, 0.0);
    for l in 1..q {
        sum = sum + temp[l as usize] * nv[l as usize];
    }
    print!("{} ", sum);
    return sum;
}

fn find_span(n:i32, p:i32, u:f64, knots:&Vec<f64>) -> i32 {
    if u == knots[(n + 1) as usize] {
        return n;
    }

    let mut low:i32 = p;
    let mut high:i32 = n + 1;
    let mut mid:i32 = (low + high) / 2;

    // The next algorithm is a binary search
    while u < knots[mid as usize] || u >= knots[(mid + 1) as usize] {
        if u < knots[mid as usize] {
            high = mid;
        } else {
            low = mid;
        }
        mid = (low + high) / 2;
    }

    return mid;
}

fn basis_funs(i:i32,u:f64,p:i32,knots:&Vec<f64>) -> Vec<f64> {

    let mut left:Vec<f64> = Vec::new();
    let mut right:Vec<f64> = Vec::new();

    let mut n:Vec<f64> = Vec::new();
    // N[0] = 1
    n.push(1.0);

    for j in 1..=p {
        left.push(u - knots[(i + 1 - j) as usize]);
        right.push(knots[(i + j) as usize] - u);
        let mut saved:f64 = 0.0;
        for r in 0..j {
            let temp:f64 = n[r as usize] / (right[(r) as usize] + left[(j - r - 1) as usize]);

            n[r as usize] = saved + right[(r) as usize] * temp;

            saved = left[(j - r - 1) as usize] * temp;
        }
        n.push(saved);
    }

    return n;
}
