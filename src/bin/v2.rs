use reticle::v2::asm::parser::AsmParser;

fn main() {
    let x = AsmParser::parse_from_str("x");
    println!("{:?}", x);
}
