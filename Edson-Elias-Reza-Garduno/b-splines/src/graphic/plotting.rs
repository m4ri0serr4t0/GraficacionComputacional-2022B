use plotters::prelude::*;
use crate::common::vec_point::Vec;
use crate::common::vec_point::Point;
use crate::common::vec_point::Transform;
use crate::common::vec_point::RangeXY;
use std::ops::Range;

pub fn plot_curve(title:&str, fname:&str, points:&Vec<Point>, curve:&Vec<Point>, width_img:u32, height_img:u32) {
    let drawing_area = BitMapBackend::new(fname, (width_img, height_img)).into_drawing_area();
	drawing_area.fill(&WHITE).unwrap();

	let x_range: Range<f64> = points.x_range();
	let y_range: Range<f64> = points.y_range();

	let width: Range<f64> = Range {
		start: 0.0, 
		end: x_range.end.ceil() + 0.5
	};

	let height = Range {
		start: 0.0, 
		end: y_range.end.ceil() + 0.5
	};

	let mut chart = ChartBuilder::on(&drawing_area)
		.caption(title, ("Arial", width_img/20))
		.set_label_area_size(LabelAreaPosition::Left, 40)
		.set_label_area_size(LabelAreaPosition::Bottom, 40)
		.build_cartesian_2d(width, height)
		.unwrap();
	chart.configure_mesh().draw().unwrap();

	// CURVA
	chart.draw_series(
		LineSeries::new(curve.to_tuples(), &RED)
	).unwrap();

	// PUNTOS DE CONTROL
	chart.draw_series(
		PointSeries::of_element(
			points.to_tuples(),
			5, &GREEN,
			&|c, s, st| {
				return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
				+ Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
				+ Text::new(format!("{:?}", c), (10, 0), ("sans-serif", width_img/100).into_font());
			}
		)
	).unwrap();
	drawing_area.present().expect("No se pudo crear el archivo");
	println!("La curva se ha guardado en {}", fname);
}