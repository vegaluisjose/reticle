use ir::errors::Error;
use ir::parser::IRParser;

fn main() -> Result<(), Error> {
    let prog = IRParser::parse_from_file("examples/eval/fsm_7.ir")?;
    println!("{}", prog);
    Ok(())
}
