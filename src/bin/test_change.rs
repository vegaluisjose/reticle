use ir::parser::IRParser;
use ir::errors::Error;

fn main() -> Result<(), Error> {
    let prog = IRParser::parse_from_file("examples/eval/fsm_7.ir")?;
    println!("{}", prog);
    Ok(())
}
