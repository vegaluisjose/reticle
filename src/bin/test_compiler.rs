use reticle::compiler::select;
use reticle::ir::parser::IRParser;
use reticle::util::errors::Error;

fn main() -> Result<(), Error> {
    let prog = IRParser::parse_from_file("examples/ir/fsm.ir")?;
    let asm = select(&prog)?;
    println!("---ir---\n{}", prog);
    println!("---asm---\n{}", asm);
    Ok(())
}
