use pat::ast::Target;
use pat::errors::Error;
use pat::parser::Parser;

fn main() -> Result<(), Error> {
    let target = Parser::parse_from_file("examples/target/patterns/lut.pat")?;
    target.serialize_to_file("lut_pat.bin");
    let des = Target::deserialize_from_file("lut_pat.bin");
    println!("{}", target);
    println!("\n\n\n{}", des);
    Ok(())
}
