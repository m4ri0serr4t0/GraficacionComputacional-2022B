use crate::common::point::Point;
use crate::common::vec_point;

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
