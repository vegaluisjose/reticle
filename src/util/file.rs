use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn create_abs_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut p = PathBuf::new();
    p.push(env!("CARGO_MANIFEST_DIR"));
    p.push(path);
    p
}

pub fn create_tmp_path<P: AsRef<Path>>(path: P) -> PathBuf {
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

pub fn write_to_tmp_file<P: AsRef<Path>>(path: P, contents: &str) -> PathBuf {
    let temp_path = create_tmp_path(path);
    write_to_file(&temp_path, contents);
    temp_path
}

pub fn read_from_tmp_file<P: AsRef<Path>>(path: P) -> String {
    let temp_path = create_tmp_path(path);
    read_to_string(&temp_path)
}

pub fn remove_tmp_file<P: AsRef<Path>>(path: P) {
    let temp_path = create_tmp_path(path);
    fs::remove_file(&temp_path).expect("Error: cannot remove temp file");
}
