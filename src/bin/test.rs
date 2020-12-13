use reticle::asm::parser::AsmParser;
use reticle::ir::parser::IRParser;
use reticle::mdl::ast as mdl;
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
    let prog = mdl::OpReg::Fdre;
    println!("{}", ir.unwrap());
    println!("{}", asm.unwrap());
    println!("{}", tdl.unwrap());
    println!("{}", prog);
}
