use reticle::asm::parser::AsmParser;
use reticle::ir::parser::IRParser;
use reticle::tdl::parser::TDLParser;

fn main() {
    let ir = IRParser::parse_from_str("main -> (y:i8) { y:i8 = const[3]; }");
    let asm = AsmParser::parse_from_str("main -> (y:i8) { y:i8 = lut_reg[3](a, b) @lut; }");
    let tdl = TDLParser::parse_from_str(
        "main[lut, 0, 1](a:i8, b:bool) -> (y:i8) { y:i8 = reg[3](a, b); }",
    );
    println!("{}", ir.unwrap());
    println!("{}", asm.unwrap());
    println!("{}", tdl.unwrap());
}
