/*
* Project: circle-code
* File: src/main.rs
* Author: Quentin de Quelen (quentin@dequelen.me)
*/

// #![feature(plugin)]
// #![plugin(clippy)]

extern crate urlencoding;

mod circle_code;

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Need arguments");
        return;
    } else if args.len() > 5 {
        println!("Too much arguments");
        return;
    }

    let url: String = env::args().nth(1).unwrap_or_else(|| "".to_string());
    let avatar: String = env::args().nth(2).unwrap_or_else(|| "https://1stamender.com/images/articlepictures/tumblrinlineo81us8Mk631tiffji1280mombasa3d.jpg".to_owned());
    let logo: String = env::args().nth(3).unwrap_or_else(|| "https://cdn.intra.42.fr/users/medium_qdequele.jpg".to_owned());
    let color: String = env::args().nth(4).unwrap_or_else(|| "#0084ff".to_string());

    circle_code::get_circle(&url, &avatar, &logo, &color, &circle_code::PointNumber::P36, "sample/image.svg")
}
