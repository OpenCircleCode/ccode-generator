
extern crate reqwest;

use std::io::Read;

pub fn image(url : String) {
    
    let mut resp = reqwest::get(url.as_str()).unwrap();
    assert!(resp.status().is_success());

    let mut content: Vec<u8> = Vec::new();
    for c in resp.bytes() {
		content.push(c.unwrap());
    };

	let content_string: String = String::from_utf8(content).unwrap();
	println!("content : {:?}", content_string);
    
}

