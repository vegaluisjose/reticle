use xir::errors::Error;
use xir::parser::Parser;

fn main() -> Result<(), Error> {
    let prog = Parser::parse_from_file("examples/xir/lut_add.xir")?;
    println!("{}", prog);
    Ok(())
}
