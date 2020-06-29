use reticle::lang::ast::*;
use reticle::passes::selection;
use serde_json;
use std::fs;

fn main() {
    let contents = fs::read_to_string("examples/dot_systolic.json")
        .expect("Error: reading the file");
    //let debug = DataType::Vector(Rc::new(DataType::UInt(8)), 3);
    //println!("{}", serde_json::to_string(&debug).expect("Error"));
    let prog: Prog = serde_json::from_str(&contents).expect("Error: deserializing");
    println!("{}", prog);
    //selection::example();
}
