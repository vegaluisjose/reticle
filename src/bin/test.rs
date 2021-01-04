use reticle::verilog::ast as verilog;
use reticle::xml::parser::MLParser;
use std::convert::TryFrom;

fn main() {
    let parsed_ml = MLParser::parse_from_file("examples/vadd.xml");
    if let Ok(prog) = parsed_ml {
        println!("{}", &prog);
        println!("{}", verilog::Module::try_from(prog).unwrap());
    } else {
        println!("{}", parsed_ml.unwrap());
    }
}
