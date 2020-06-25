use reticle::lang::ast::*;
use serde_json;
use std::fs;

fn main() {
    let contents = fs::read_to_string("examples/add.json")
        .expect("Error: reading the file");
    let prog: Prog = serde_json::from_str(&contents).expect("Error: deserializing");
    println!("{:?}", prog);
}
