use reticle::lang::ast::*;
use reticle::passes::binding;
use serde_json;
use std::fs;

fn main() {
    let contents = fs::read_to_string("examples/add.json").expect("Error: reading the file");
    let prog: Prog = serde_json::from_str(&contents).expect("Error: deserializing");
    println!("{}", &prog);
    let mut dag = binding::DAG::new();
    dag.from_ast(&prog);
    let code = dag.get_root();
    let code_opt = binding::opt(&code, &binding::get_patterns());
    println!("opt");
    for node in code_opt.iter() {
        println!("{:?}", node);
    }
}
