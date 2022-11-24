use plotters::prelude::*;
use crate::geometry::vec_points::Vec;
use crate::geometry::vec_points::Point;
use crate::geometry::vec_points::{Transform};
use crate::geometry::vec_points::{RangeXY};
use std::ops::Range;

pub fn plot_graph(title:&str, fname:&str, dots:&Vec<Point>, graph:&Vec<Point>) {
	let drawing_area = BitMapBackend::new(fname, (500, 500)).into_drawing_area();
	drawing_area.fill(&WHITE).unwrap();





	let x_range = dots.x_range();
	let y_range = dots.y_range();
	let width = Range {
		start: 0.0, 
		end: x_range.end.ceil()
	};
	let height = Range {
		start: 0.0, 
		end: y_range.end.ceil()
	};







	let mut chart = ChartBuilder::on(&drawing_area)
		.caption(title, ("tl", 20))
		.set_label_area_size(LabelAreaPosition::Left, 40)
		.set_label_area_size(LabelAreaPosition::Bottom, 40)
		.build_cartesian_2d(width, height)
		.unwrap();
	chart.configure_mesh().draw().unwrap();


















	// CURVE
	chart.draw_series(
		LineSeries::new(graph.to_tuples(), &BLACK)
	).unwrap();

	// POINTS
	chart.draw_series(
		PointSeries::of_element(
			dots.to_tuples(),
			5, &BLACK,
			&|c, s, st| {
				return EmptyElement::at(c) 
				+ Circle::new((0,0),s,st.filled()) 
				+ Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
			}
		)
	).unwrap();

	println!("\tCreando {}...", fname);
	drawing_area.present().expect("ERROR");
}









pub fn plot_pollygon(fname:&str, dots:&Vec<Point>, graph:&Vec<Point>) {
	let drawing_area = BitMapBackend::new(fname, (500, 500)).into_drawing_area();
	drawing_area.fill(&WHITE).unwrap();

	let x_range = dots.x_range();
	let y_range = dots.y_range();

	let width = Range {
		start: 0.0, 
		end: x_range.end.ceil()
	};
	let height = Range {
		start: 0.0, 
		end: y_range.end.ceil()
	};







	let mut chart = ChartBuilder::on(&drawing_area)
		.build_cartesian_2d(width, height)
		.unwrap();

	chart.draw_series(std::iter::once(Polygon::new(graph.to_tuples().clone(), &BLACK,))).unwrap();







	println!("\tCreando {}...", fname);
	drawing_area.present().expect("ERROR");
}