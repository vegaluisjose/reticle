use reticle::ml::parser::MLParser;
use reticle::verilog::ast as verilog;
use std::convert::TryFrom;

fn main() {
    let parsed_ml = MLParser::parse_from_file("examples/add.rml");
    if let Ok(prog) = parsed_ml {
        println!("{}", &prog);
        println!("{}", verilog::Module::try_from(prog).unwrap());
    }
}
