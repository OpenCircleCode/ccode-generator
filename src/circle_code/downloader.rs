/*
* Project: circle-code
* File: circle_code/downloader.rs
* Author: Quentin de Quelen (quentin@dequelen.me)
*/

extern crate reqwest;
extern crate base64;

use std::io::Read;
use self::base64::{encode};

pub fn image(url : &str) -> String {

    let mut resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let mut content: Vec<u8> = Vec::new();
    let _len = resp.read_to_end(&mut content);

    encode(&content)
}

