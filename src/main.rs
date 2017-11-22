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

    let mut url: String = "".to_string();
    let mut avatar: String = "https://cdn.intra.42.fr/users/medium_qdequele.jpg".to_owned();
    let mut logo: String = "https://cdn.intra.42.fr/users/medium_qdequele.jpg".to_owned();
    let mut color: String = "#0084ff".to_string();

    if let Some(arg1) = env::args().nth(1) {
        url = arg1;
    }
    if let Some(arg2) = env::args().nth(2) {
        avatar = arg2;
    }
    if let Some(arg3) = env::args().nth(3) {
        logo = arg3;
    }
    if let Some(arg4) = env::args().nth(4) {
        color = arg4;
    }

    circle_code::get_circle(&url, &avatar, &logo, &color, circle_code::PointNumber::P72)
}
