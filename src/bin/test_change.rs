use xim::errors::Error;
use xim::parser::Parser;

fn main() -> Result<(), Error> {
    let target = Parser::parse_from_file("examples/target/implementations/ultrascale/lut.xim")?;
    println!("{}", target);
    Ok(())
}
