/*
* Project: circle-code
* File: circle_code/hash.rs
* Author: Quentin de Quelen (quentin@dequelen.me)
*/

extern crate md5;

use std::str;

pub fn get_code(text: &str) -> Vec<u32> {
    sting_to_binary(text)
}

fn sting_to_binary(text: &str) -> Vec<u32> {
    let binary_string = string_to_binary_string(text);
    binary_string.chars()
                .into_iter()
                .map(|c| { if c == '0' {0} else {1} })
                .collect()
}

fn string_to_binary_string(text: &str) -> String {
    let encoded_vec: Vec<String> = text.chars()
                                    .into_iter()
                                    .map(|s| format!("{:08b}", s as u8))
                                    .collect();
    println!("binary string : {}", encoded_vec.join(""));
    encoded_vec.join("")
}

#[test]
fn test_string_to_binary_string_1() {
    assert!(string_to_binary_string("http://github.com/qdequele")
    == "0110100001110100011101000111000000111010001011110010111101100111011010010111010001101000011\
    10101011000100010111001100011011011110110110100101111011100010110010001100101011100010111010101\
    1001010110110001100101");
}

#[test]
fn test_string_to_binary_string_2() {
    assert!(string_to_binary_string("https://doc.rust-lang.org")
    == "0110100001110100011101000111000001110011001110100010111100101111011001000110111101100011001\
    01110011100100111010101110011011101000010110101101100011000010110111001100111001011100110111101\
    11001001100111");
}

#[test]
fn test_string_to_binary_string_3() {
    assert!(string_to_binary_string("bit.ly/2B6xljP")
    == "0110001001101001011101000010111001101100011110010010111100110010010000100011011001111000011\
    011000110101001010000");
}
