use plotters::prelude::*;
use crate::geometry::vec_points::Vec;
use crate::geometry::vec_points::Point;
use crate::geometry::vec_points::{Transform};
use crate::geometry::vec_points::{RangeXY};
use std::ops::Range;

pub fn plot_graph(title:&str, fname:&str, dots:&Vec<Point>, graph:&Vec<Point>) {

	let drawing_area = BitMapBackend::new(fname, (800, 800)).into_drawing_area();
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
		.caption(title, ("Arial", 30))
		.set_label_area_size(LabelAreaPosition::Left, 40)
		.set_label_area_size(LabelAreaPosition::Bottom, 40)
		.build_cartesian_2d(width, height)
		.unwrap();
	chart.configure_mesh().draw().unwrap();

	// CURVA
	chart.draw_series(
		LineSeries::new(graph.to_tuples(), &RED)
	).unwrap();

	// PUNTOS DE CONTROL
	chart.draw_series(
		PointSeries::of_element(
			dots.to_tuples(),
			5, &GREEN,
			&|c, s, st| {
				return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
				+ Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
				+ Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
			}
		)
	).unwrap();

	println!("\tCreando archivo {}...", fname);
	drawing_area.present().expect("No se pudo crear el archivo");
}