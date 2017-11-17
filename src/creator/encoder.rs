/*
 * Project: circle-code
 * File: src/hash.rs
 * Author: Quentin de Quelen (quentin@dequelen.me)
 */

extern crate encoding;
extern crate md5;

use std::str;
use self::encoding::{Encoding, EncoderTrap};
use self::encoding::all::ISO_8859_1;

pub fn code(text: &str) -> String {

	let mut bytes = Vec::new();

	match ISO_8859_1.encode_to(&text, EncoderTrap::Ignore, &mut bytes) {
		Ok(_) => {},
		Err(e) => println!("{}", e)
	}

	let md5_text = md5::compute(bytes.clone());

	let encoded_string: Vec<String> = md5_text.into_iter().map(|s| format!("{:08b}", s)).collect();
	return encoded_string.join("")
}

