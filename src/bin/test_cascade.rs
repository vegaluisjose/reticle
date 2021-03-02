use reticle::asm::ast as asm;
use reticle::ir::parser::IRParser;
use reticle::optimizer::cascader::cascade;
use reticle::placer::place_from_prog;
use reticle::util::errors::Error;
use reticle::verilog::ast as verilog;
use reticle::xl::ast as xl;
use std::convert::TryFrom;

fn main() -> Result<(), Error> {
    let prog = IRParser::parse_from_file("examples/ir/tdot.ir")?;
    let asm = asm::Prog::try_from(prog.clone())?;
    let asm = cascade(&asm)?;
    let placed = place_from_prog(&asm)?;
    let xl = xl::Prog::try_from(placed.clone())?;
    let module = verilog::Module::try_from(xl.clone())?;
    println!("---ir---\n{}", prog);
    println!("---asm---\n{}", asm);
    println!("---placed---\n{}", placed);
    println!("---xl---\n{}", xl);
    println!("---verilog---\n{}", module);
    Ok(())
}
