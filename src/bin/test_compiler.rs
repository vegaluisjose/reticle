use reticle::compiler::select;
use reticle::ir::parser::IRParser;
use reticle::placer::place_lut_from_prog;
use reticle::util::errors::Error;
use reticle::xl::translate::test;

fn main() -> Result<(), Error> {
    let prog = IRParser::parse_from_file("examples/ir/fsm.ir")?;
    let asm = select(&prog)?;
    let placed = place_lut_from_prog(&asm)?;
    let xl = test(&placed)?;
    println!("---ir---\n{}", prog);
    println!("---asm---\n{}", asm);
    println!("---placed---\n{}", placed);
    println!("---xl---\n{}", xl);
    Ok(())
}
