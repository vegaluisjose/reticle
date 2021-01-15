use reticle::ir::parser::IRParser;
use reticle::verilog::ast as verilog;
use std::convert::TryFrom;

fn main() {
    let prog = IRParser::parse_from_file("examples/add.ir");
    if let Ok(p) = prog {
        println!("{}", &p);
        println!("{}", verilog::Module::try_from(p).unwrap());
    }
    // if let Ok(prog) = parsed_ml {
    //     println!("{}", &prog);
    //     println!("{}", verilog::Module::try_from(prog).unwrap());
    // } else {
    //     println!("{}", parsed_ml.unwrap());
    // }
}
