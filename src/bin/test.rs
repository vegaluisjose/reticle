use reticle::asm::parser::AsmParser;
use reticle::ir::parser::IRParser;
use reticle::ml::parser::MLParser;
use reticle::tdl::parser::TDLParser;

fn main() {
    let ir =
        IRParser::parse_from_str("main(a:bool, b:bool) -> (y:bool) { y:bool = reg[0](a, b); }");
    let asm = AsmParser::parse_from_str(
        "main(a:bool, b:bool) -> (y:bool) { y:bool = lut_reg(a, b) @lut(x+1,y); }",
    );
    let tdl = TDLParser::parse_from_str(
        "lut_reg[lut, 0, 1](a:bool, b:bool) -> (y:bool) { y:bool = reg[0](a, b); }",
    );
    let mach = MLParser::parse_from_str(
        "def main() -> () { y:i8 = dsp[ra=1](a,b) @(1,2); }",
    );
    println!("{}", ir.unwrap());
    println!("{}", asm.unwrap());
    println!("{}", tdl.unwrap());
    println!("{}", mach.unwrap());
}
