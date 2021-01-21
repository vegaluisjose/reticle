use reticle::ir::dfg::Dfg;
use reticle::ir::parser::IRParser;

fn main() {
    let prog = IRParser::parse_from_file("examples/muladd_reg.ir");
    if let Ok(p) = prog {
        let dfg = Dfg::from(p);
        println!("{}", dfg);
    }
}
