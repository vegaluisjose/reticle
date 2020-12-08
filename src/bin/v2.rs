use reticle::v2::asm::parser::AsmParser;
use reticle::v2::ir::parser::IRParser;

fn main() {
    let ir = IRParser::parse_from_str("main -> (y:i8) { y:i8 = const[3]; }");
    let asm = AsmParser::parse_from_str("main -> (y:i8) { y:i8 = lut_reg[3](a, b) @lut; }");
    println!("{}", ir.unwrap());
    println!("{}", asm.unwrap());
}
