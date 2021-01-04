use reticle::verilog::ast as verilog;
use reticle::xl::parser::MLParser;
use std::convert::TryFrom;

fn main() {
    let parsed_ml = MLParser::parse_from_file("examples/vadd.xl");
    if let Ok(prog) = parsed_ml {
        println!("{}", &prog);
        println!("{}", verilog::Module::try_from(prog).unwrap());
    } else {
        println!("{}", parsed_ml.unwrap());
    }
}
