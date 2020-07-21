use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_to_string<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(path).expect("Error: openning the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error: reading the file");
    contents
}