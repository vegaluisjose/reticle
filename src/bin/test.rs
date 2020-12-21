use reticle::ir::parser::IRParser;
use reticle::ml::parser::MLParser;
use reticle::verilog::ast as verilog;

fn main() {
    let parsed_prog =
        IRParser::parse_from_str("def main(a:bool, b:bool) -> (y:bool) { y:bool = reg[0](a, b); }");
    let parsed_ml = MLParser::parse_from_file("examples/add.rml");
    if let Ok(prog) = parsed_prog {
        if let Some(main) = prog.get("main") {
            println!("{}", verilog::Module::from(main.sig().clone()));
        }
    }
    if let Ok(prog) = parsed_ml {
        println!("{}", verilog::Module::from(prog.sig().clone()));
    }
}
