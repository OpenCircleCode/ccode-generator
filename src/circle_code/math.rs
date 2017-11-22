/*
 * Project: circle-code
 * File: circle_code/math.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

use std::f64;

pub struct CartesianCoord {
	pub x: f64,
	pub y: f64
}

pub fn polar_to_cartesian(center_x: f64, center_y: f64, radius: f64, angle: f64) -> CartesianCoord {
	CartesianCoord {
		x: center_x + (radius * angle.to_radians().cos()),
		y: center_y + (radius * angle.to_radians().sin())
	}
}
