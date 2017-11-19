

extern crate reqwest;

use std::io::Read;

pub fn download_from_url(url: String) -> String {
	let mut content = String::new();

	println!("{}", url);
	let mut resp = reqwest::get(url.as_str()).unwrap();

	println!("Status: {}", resp.status());
	println!("Headers:\n{}", resp.headers());

	resp.read_to_string(&mut content).unwrap();

	println!("{:?}", content);

	content
}
