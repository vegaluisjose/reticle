use reticle::lang::ast::*;
use reticle::passes::binding;
use serde_json;
use std::fs;

fn main() {
    let contents = fs::read_to_string("examples/dot_systolic.json")
        .expect("Error: reading the file");
    let prog: Prog = serde_json::from_str(&contents).expect("Error: deserializing");
    println!("{}", prog);
    let dag = binding::DAG::new();
    dag.from_ast(&prog);
}
