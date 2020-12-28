use reticle::ir::parser::IRParser;
use reticle::ml::parser::MLParser;
use reticle::verilog::ast as verilog;
use std::convert::TryFrom;

fn main() {
    let parsed_prog =
        IRParser::parse_from_str("def main(a:bool, b:bool) -> (y:bool) { y:bool = const[1]; }");
    let parsed_ml = MLParser::parse_from_file("examples/add.rml");
    if let Ok(prog) = parsed_prog {
        if let Some(main) = prog.get("main") {
            println!("{}", verilog::Module::from(main.clone()));
        }
    }
    if let Ok(prog) = parsed_ml {
        println!("{}", verilog::Module::try_from(prog).unwrap());
    }
}
