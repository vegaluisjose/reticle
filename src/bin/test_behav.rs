use reticle::ir::parser::IRParser;
use reticle::util::errors::Error;
use reticle::verilog::ast as verilog;
use std::convert::TryFrom;

fn main() -> Result<(), Error> {
    let prog = IRParser::parse_from_file("examples/ir/tdot.ir")?;
    let module = verilog::Module::try_from(prog.clone())?;
    println!("---ir---\n{}", prog);
    println!("---verilog---\n{}", module);
    Ok(())
}
