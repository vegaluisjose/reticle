use reticle::ir::dfg::Dfg;
use reticle::ir::parser::IRParser;
use std::convert::TryFrom;

fn main() {
    let prog = IRParser::parse_from_file("examples/fsm.ir");
    if let Ok(p) = prog {
        println!("Program:\n{}", &p);
        let dfg = Dfg::try_from(p).unwrap();
        println!("Graphviz:\n{}", dfg);
    }
}
