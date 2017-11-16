/*
 * Project: circle-code
 * File: src/svg.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

use math;

pub struct Arc {
	start: f64,
	len: f64,
	level: f64
}

pub fn describe_arc(x: f64, y: f64, radius: f64, start_angle: f64, end_angle:f64) {
	let start: math::CartesianCoord= math::polar_to_cartesian(x, y, radius, end_angle);
	let end: math::CartesianCoord = math::polar_to_cartesian(x, y, radius, start_angle);

	let mut largeArcFlag = "";
	if end_angle - start_angle <= 180_f64 {
		largeArcFlag = "0";
	} else {
		largeArcFlag = "1";
	}
}


pub fn generate_arc(code: String) -> Vec<Arc> {
	let len = code.len();
}
