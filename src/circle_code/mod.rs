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

pub fn get_circle(url: &str, avatar: &str, logo: &str, color: &str, number_of_points: &PointNumber, filename: &str) {

    let hashed_string = encoder::get_code(url);

    let nb_points = SPointNumber::new(number_of_points);

    let arcs: Vec<Arc> = calculate_arcs(&hashed_string, &nb_points);

    let svg = svg::generate_svg(&arcs, avatar, logo, color, &nb_points);

    file::save(svg, filename);
}

#[test]
fn test_full_1() {
    get_circle("ÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿ", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P120, "sample/120/test_full_1.svg");
    get_circle("ÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿ", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P90, "sample/90/test_full_1.svg");
    get_circle("ÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿ", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P72, "sample/72/test_full_1.svg");
    get_circle("ÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿ", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P60, "sample/60/test_full_1.svg");
    get_circle("ÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿÿ", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P36, "sample/36/test_full_1.svg");
}
#[test]
fn test_full_0() {
    get_circle("                                                      ", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P120, "sample/120/test_full_0.svg");
    get_circle("                                                      ", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P90, "sample/90/test_full_0.svg");
    get_circle("                                                      ", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P72, "sample/72/test_full_0.svg");
    get_circle("                                                      ", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P60, "sample/60/test_full_0.svg");
    get_circle("                                                      ", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P36, "sample/36/test_full_0.svg");
}
#[test]
fn test_alternate_01() {
    get_circle("UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P120, "sample/120/test_alternate_01.svg");
    get_circle("UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P90, "sample/90/test_alternate_01.svg");
    get_circle("UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P72, "sample/72/test_alternate_01.svg");
    get_circle("UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P60, "sample/60/test_alternate_01.svg");
    get_circle("UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P36, "sample/36/test_alternate_01.svg");
}
#[test]
fn test_alternate_10() {
    get_circle("ªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªª", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P120, "sample/120/test_alternate_10.svg");
    get_circle("ªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªª", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P90, "sample/90/test_alternate_10.svg");
    get_circle("ªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªª", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P72, "sample/72/test_alternate_10.svg");
    get_circle("ªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªª", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P60, "sample/60/test_alternate_10.svg");
    get_circle("ªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªªª", "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg", "", "#0084ff", &PointNumber::P36, "sample/36/test_alternate_10.svg");
}
