use reticle::asm::parser::AsmParser;
use reticle::optimizer::cascader::cascade;
use reticle::util::errors::Error;

fn main() -> Result<(), Error> {
    let p0 = AsmParser::parse_from_file("examples/asm/tdot.asm")?;
    let p1 = cascade(&p0)?;
    println!("input\n{}\n", p0);
    println!("output\n{}\n", p1);
    Ok(())
}
