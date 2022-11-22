use crate::common::vec_point::Point;
use crate::common::vec_point::RangeXY;
use crate::common::vec_point::Transform;
use crate::common::vec_point::Vec;
use crate::common::vec_point3d::Point3D;
use crate::common::vec_point3d::Transform as Transform3D;
use plotters::prelude::*;
use std::ops::Range;

pub fn plot_curve(title: &str, points: &Vec<Point>, curve: &Vec<Point>) {
    const WIDTH_IMG: u32 = 1200;
    const HEIGHT_IMG: u32 = 1200;
    let drawing_area = BitMapBackend::new(title, (WIDTH_IMG, HEIGHT_IMG)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let x_range: Range<f64> = points.x_range();
    let y_range: Range<f64> = points.y_range();

    let width: Range<f64> = Range {
        start: 0.0,
        end: x_range.end.ceil() + 0.5,
    };

    let height = Range {
        start: 0.0,
        end: y_range.end.ceil() + 0.5,
    };

    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(width, height)
        .unwrap();
    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(curve.to_tuples(), &RED))
        .unwrap();

    chart
        .draw_series(PointSeries::of_element(
            points.to_tuples(),
            5,
            &GREEN,
            &|c, s, st| {
                return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
				+ Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
				+ Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
            },
        ))
        .unwrap();

    drawing_area.present().expect("No se pudo crear el archivo");
    println!("La curva se ha guardado en {}", title);
}

pub fn plot_polygon(title: &str, points: &Vec<Point>, curve: &Vec<Point>) {
	let drawing_area = BitMapBackend::new(title, (800, 800)).into_drawing_area();
	drawing_area.fill(&WHITE).unwrap();

	let x_range = points.x_range();
	let y_range = points.y_range();

	let width = Range {
		start: 0.0, 
		end: x_range.end.ceil() + 1.0
	};

	let height = Range {
		start: 0.0, 
		end: y_range.end.ceil() + 1.0
	};

	let mut chart = ChartBuilder::on(&drawing_area)
		.build_cartesian_2d(width, height)
		.unwrap();

	chart.draw_series(std::iter::once(Polygon::new(curve.to_tuples().clone(), &BLACK,))).unwrap();

	println!("\tCreando archivo {}...", title);
	drawing_area.present().expect("No se pudo crear el archivo");
}

pub fn plot_surface(title: &str, points: &Vec<Vec<Point3D>>, surface: &Vec<Point3D>) {
    const WIDTH_IMG: u32 = 1200;
    const HEIGHT_IMG: u32 = 1200;
    let drawing_area = BitMapBackend::new(title, (WIDTH_IMG, HEIGHT_IMG)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut min_point: Point3D = points[0][0];
    let mut max_point: Point3D = points[0][0];

    for i in points {
        for j in i {
            if j.x < min_point.x {
                min_point.x = j.x;
            }
            if j.y < min_point.y {
                min_point.y = j.y;
            }
            if j.z < min_point.z {
                min_point.z = j.z;
            }
            if j.x > max_point.x {
                max_point.x = j.x;
            }
            if j.y > max_point.y {
                max_point.y = j.y;
            }
            if j.z > max_point.z {
                max_point.z = j.z;
            }
        }
    }

    let width: Range<f64> = Range {
        start: min_point.x - 0.5,
        end: max_point.x + 0.5,
    };

    let height = Range {
        start: min_point.y - 0.5,
        end: max_point.y + 0.5,
    };

    let z = Range {
        start: min_point.z - 0.5,
        end: max_point.z + 0.5,
    };

    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_3d(width, height, z)
        .unwrap();

    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()
		.unwrap();

    // SUPERFICIE
    chart
        .draw_series(LineSeries::new(surface.to_tuples(), &BLACK))
        .unwrap();

    drawing_area.present().expect("No se pudo crear el archivo");
    println!("La curva se ha guardado en {}", title);
}
