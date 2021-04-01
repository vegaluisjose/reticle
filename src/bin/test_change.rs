use pat::errors::Error;
use pat::parser::Parser;

fn main() -> Result<(), Error> {
    let target = Parser::parse_from_file("examples/target/patterns/lut.pat")?;
    println!("{}", target);
    Ok(())
}
