use reticle::ir::parser::IRParser;
use reticle::verilog::ast as verilog;
use std::convert::TryFrom;

fn main() {
    let prog = IRParser::parse_from_file("examples/add_reg.ir");
    if let Ok(p) = prog {
        println!("{}", &p);
        if let Ok(v) = verilog::Module::try_from(p) {
            println!("{}", v);
        }
    }
}
