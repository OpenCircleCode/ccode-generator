
use std::env;

mod math;
mod svg;

fn main() {

	let args: Vec<String> = env::args().collect();

	if args.len() == 1 {
		println!("Need arguments");
		return;
	} else if args.len() > 2 {
		println!("Too mutch arguments");
		return;
	}

    math::polar_to_cartesian();
    svg::describe_arc();

}