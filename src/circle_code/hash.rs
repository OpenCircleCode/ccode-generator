/*
* Project: circle-code
* File: circle_code/hash.rs
* Author: Quentin de Quelen (quentin@dequelen.me)
*/

extern crate md5;

use std::str;

pub fn get_hash(text: &str) -> Vec<u32> {

    println!("text {}", text);

    let md5_text = text;//format!("{:x}", md5::compute(text.as_bytes()));

    println!("md5 : {}", md5_text);

    sting_to_binary(&md5_text)
}

fn sting_to_binary(text: &str) -> Vec<u32> {
    let binary_string = string_to_binary_string(text);
    binary_string.chars().into_iter().map(|c| { if c == '0' {0} else {1} }).collect()
}

fn string_to_binary_string(text: &str) -> String {
    let encoded_vec: Vec<String> = text.chars().into_iter().map(|s| format!("{:08b}", s as u8)).collect();
    println!("binary string : {}", encoded_vec.join(""));
    encoded_vec.join("")
}
