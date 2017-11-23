/*
* Project: circle-code
* File: circle_code/mod.rs
* Author: Quentin de Quelen (quentin@dequelen.me)
*/

mod encoder;
mod svg;
mod downloader;
mod math;
mod constructor;
mod file;

use self::constructor::{Arc, calculate_arcs};
pub use self::svg::{PointNumber, SPointNumber};

static DATA_FILE          : &'static str = "sample/image.svg";
static DATA_TEST_FILE     : &'static str = "sample/image_test.svg";

pub fn get_circle(url: &str, avatar: &str, logo: &str, color: &str, number_of_points: &PointNumber) {

    let hashed_string = encoder::get_code(url);

    let nb_points = SPointNumber::new(number_of_points);

    let arcs: Vec<Arc> = calculate_arcs(&hashed_string, &nb_points);
    let canvas: Vec<Arc>  =  svg::generate_canvas();

    let svg = svg::generate_svg(&arcs, avatar, logo, color, &nb_points);
    let svg_test = svg::generate_svg_test(&arcs, &canvas, avatar, logo, color, &nb_points);

    file::save(svg, DATA_FILE);
    file::save(svg_test, DATA_TEST_FILE);
}
