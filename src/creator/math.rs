/*
 * Project: circle-code
 * File: src/math.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

use std::f64;

pub struct CartesianCoord {
	pub x: f64,
	pub y: f64
}

pub fn polar_to_cartesian(center_x: f64, center_y: f64, radius: f64, angle: f64) -> CartesianCoord {
	let angle_radians = (angle - 90_f64) * f64::consts::PI / 180_f64;

	CartesianCoord {
		x: center_x + (radius * angle_radians.cos()),
		y: center_y + (radius * angle_radians.sin())
	}
}

