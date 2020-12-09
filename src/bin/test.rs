use reticle::asm::parser::AsmParser;
use reticle::ir::parser::IRParser;
use reticle::tdl::parser::TDLParser;

fn main() {
    let ir =
        IRParser::parse_from_str("main(a:bool, b:bool) -> (y:bool) { y:bool = reg[0](a, b); }");
    let asm = AsmParser::parse_from_str(
        "main(a:bool, b:bool) -> (y:bool) { y:bool = lut_reg(a, b) @lut; }",
    );
    let tdl = TDLParser::parse_from_str(
        "lut_reg[lut, 0, 1](a:bool, b:bool) -> (y:bool) { y:bool = reg[0](a, b); }",
    );
    let pdl =
        String::from("lut_reg(a:bool, b:bool) -> (y:bool) { y:bool = fdre(a, b) @bel(x, y); }");
    println!("{}", ir.unwrap());
    println!("{}", asm.unwrap());
    println!("{}", tdl.unwrap());
    println!("{}", pdl);
}
