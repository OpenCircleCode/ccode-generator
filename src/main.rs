/*
 * Project: circle-code
 * File: src/main.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

use std::env;

mod math;
mod svg;
mod encoder;

fn main() {

	let args: Vec<String> = env::args().collect();

	if args.len() == 1 {
		println!("Need arguments");
		return;
	} else if args.len() > 2 {
		println!("Too mutch arguments");
		return;
	}

	encoder::encode(&"https://github.com/qdequele/");
	svg::describe_arc(150_f64, 150_f64, 80_f64, 0_f64, 0_f64);

}
