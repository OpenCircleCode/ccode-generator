/*
 * Project: circle-code
 * File: src/hash.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

extern crate encoding;

use self::encoding::{Encoding, EncoderTrap, DecoderTrap};
use self::encoding::all::ISO_8859_1;
use std::str;

pub fn encode(text: &str) -> Vec<u8>{

	let mut bytes = Vec::new();
	let mut chars = String::new();

	match ISO_8859_1.encode_to(text, EncoderTrap::Ignore, &mut bytes) {
		Ok(_) => {},
		Err(e) => println!("{}", e)
	}
	for bit in bytes.clone() {
		println!("{:08b}", bit);
	}
	print_line(bytes.clone());
	return bytes
}

pub fn print_line(vec: Vec<u8>) {
	for bit in vec {
		let dash = str::replace(&str::replace(&format!("{:08b}", bit), "0", " "), "1", "-");
		print!("{}", dash);
	}
}
