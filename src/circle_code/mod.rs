
mod hash;
mod svg;
mod downloader;
mod math;
mod constructor;
mod file;

use self::constructor::{Arc, calculate_arcs};

static DATA_FILE		: &'static str = "sample/image.svg";
static DATA_TEST_FILE	: &'static str = "sample/image_test.svg";

pub fn get_circle(url: &str, avatar: &str, logo: &str, color: &str) {

    let hashed_string = hash::get_hash(url);

    let arcs: Vec<Arc> = calculate_arcs(&hashed_string);
    let canvas: Vec<Arc>  =  svg::generate_canvas();

    let svg = svg::generate_svg(&arcs, avatar, logo, color);
    let svg_test = svg::generate_svg_test(&arcs, &canvas, avatar, logo, color);

    file::save(svg, DATA_FILE);
    file::save(svg_test, DATA_TEST_FILE);
}