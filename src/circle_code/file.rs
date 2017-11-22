/*
* Project: circle-code
* File: circle_code/file.rs
* Author: Quentin de Quelen (quentin@dequelen.me)
*/

use std::fs::File;
use std::error::Error;
use std::path::Path;
use std::io::prelude::*;

pub fn save(datas: Vec<String>, filename :&str) {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => {
            println!("couldn't create {}: {}",
                display,
                why.description());
            return
        },
        Ok(file) => file,
    };

    for data in datas {
        if let Err(why) = file.write_all(data.as_bytes()) {
            println!("couldn't write to {}: {}",
                display,
                why.description());
            return
        }
    }
}
