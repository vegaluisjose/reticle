use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn create_absolute<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut p = PathBuf::new();
    p.push(env!("CARGO_MANIFEST_DIR"));
    p.push(path);
    p
}

pub fn create_tempfile_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut p = env::temp_dir();
    p.push(path);
    p
}

pub fn read_to_string<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(path).expect("Error: openning the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error: reading the file");
    contents
}

pub fn write_to_file<P: AsRef<Path>>(path: P, contents: &str) {
    let mut file = File::create(path).expect("Error: cannot create a file");
    file.write_all(contents.as_bytes())
        .expect("Error: writing the file");
}

pub fn write_to_tempfile<P: AsRef<Path>>(path: P, contents: &str) -> PathBuf {
    let temp_path = create_tempfile_path(path);
    let mut file = File::create(&temp_path).expect("Error: cannot create a file");
    file.write_all(contents.as_bytes())
        .expect("Error: writing the file");
    temp_path
}
