use reticle::ir::dfg::Dfg;
use reticle::ir::parser::IRParser;
use std::convert::TryFrom;

fn main() {
    let prog = IRParser::parse_from_file("examples/muladd_reg.ir");
    if let Ok(p) = prog {
        let dfg = Dfg::try_from(p).unwrap();
        println!("{}", dfg);
    }
}
