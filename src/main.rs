/*
 * Project: circle-code
 * File: src/main.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

use std::env;

mod creator;

use creator::{encoder, svg};

fn main() {

	let args: Vec<String> = env::args().collect();

	if args.len() == 1 {
		println!("Need arguments");
		return;
	} else if args.len() > 2 {
		println!("Too much arguments");
		return;
	}

	let coded_string = encoder::code(&args[1]);
	let arcs: Vec<svg::Arc> = svg::calculate_arcs(coded_string);
	svg::generate_svg(arcs, &"https://cdn.intra.42.fr/users/medium_qdequele.jpg", &"", &"#0084ff");

}
