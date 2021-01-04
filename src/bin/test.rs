use reticle::verilog::ast as verilog;
use reticle::xl::parser::XLParser;
use std::convert::TryFrom;

fn main() {
    let parsed_ml = XLParser::parse_from_file("examples/vadd.xl");
    if let Ok(prog) = parsed_ml {
        println!("{}", &prog);
        println!("{}", verilog::Module::try_from(prog).unwrap());
    } else {
        println!("{}", parsed_ml.unwrap());
    }
}
