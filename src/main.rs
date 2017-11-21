/*
 * Project: circle-code
 * File: src/main.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */


extern crate urlencoding;

mod creator;

use std::env;
use creator::{encoder, svg, downloader};
use urlencoding::encode;

fn main() {

	let args: Vec<String> = env::args().collect();

	if args.len() == 1 {
		println!("Need arguments");
		return;
	} else if args.len() > 5 {
		println!("Too much arguments");
		return;
	}

	let mut url: String = "".to_string();
	let mut avatar: String = encode(&"https://avatars0.githubusercontent.com/u/6064892?s=460&v=4");
	let mut logo: String = encode(&"https://avatars2.githubusercontent.com/u/33703450?s=200&v=4");
	let mut color: String = "#0084ff".to_string();

	if let Some(arg1) = env::args().nth(1) {
		url = encode(&arg1);
	}
	if let Some(arg2) = env::args().nth(2) {
		avatar = arg2.replace("&", "%26");
	}
	if let Some(arg3) = env::args().nth(3) {
		logo = arg3.replace("&", "%26");
	}
	if let Some(arg4) = env::args().nth(4) {
		color = arg4;
	}

	let coded_string = encoder::code(&url);
	let arcs: Vec<svg::Arc> = svg::calculate_arcs(coded_string);

	let canvas: Vec<svg::Arc>  =  svg::generate_canvas();

	svg::generate_svg(arcs.clone(), &avatar, &logo, &color);
	svg::generate_svg_test(arcs.clone(), canvas, &avatar, &logo, &color);

	downloader::image("http://avatars2.githubusercontent.com/u/33703450?s=200&v=4".to_owned());

}
