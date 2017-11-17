/*
 * Project: circle-code
 * File: src/svg.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

static NB_LINES: u32 = 4_u32;
static PRECISION: f64 = 10_f64;

use super::math;

#[derive(Debug, Clone, Copy)]
pub struct Arc {
	pub start: u32,
	pub len: u32,
	pub level: u32
}

pub fn describe_arc(x: f64, y: f64, radius: f64, start_angle: f64, end_angle:f64) -> String{
	let start: math::CartesianCoord= math::polar_to_cartesian(x, y, radius, end_angle);
	let end: math::CartesianCoord = math::polar_to_cartesian(x, y, radius, start_angle);

	let mut largeArcFlag = "";
	if end_angle - start_angle <= 180_f64 {
		largeArcFlag = "0";
	} else {
		largeArcFlag = "1";
	}

	format!("M {} {} A {} {} 0 {} 0 {} {}", start.x, start.y, radius, radius, largeArcFlag, end.x, end.y)
}


pub fn generate_arcs(code: String) -> Vec<Arc> {
	let len = code.len();
	let mut arcs: Vec<Arc> = Vec::new();

	println!("{}", code);

	let mut start: u32 = 0;
	let mut len:u32 = 0;
	let mut level:u32 = 0;

	for c in code.chars() {
		if start + len == 36 {
			arcs.push(Arc{
				start: start,
				len: len,
				level: level
			});
			start = 0;
			len = 0;
			level +=1;
		}
		if c == '0' {
			if len != 0 {
				arcs.push(Arc{
					start: start,
					len: len,
					level: level
				});
				start += len;
				len = 0;
			}
			start += 1;
		} else if c == '1' {
			len += 1;
		}
	}

	for arc in arcs.clone() {
		let desc = describe_arc(150_f64, 150_f64, 80_f64, (arc.start * 10) as f64, ((arc.start + arc.len) * 10) as f64);
		println!("{:?} | {}", arc, desc);
	}

	arcs
}
