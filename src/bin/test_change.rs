use asm::errors::Error;
use asm::parser::Parser;

fn main() -> Result<(), Error> {
    let prog = Parser::parse_from_file("examples/asm/fsm.asm")?;
    println!("{}", prog);
    Ok(())
}
