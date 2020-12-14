use reticle::asm::parser::AsmParser;
use reticle::ir::parser::IRParser;
use reticle::ml::ast as ml;
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
    let loc = ml::Loc {
        bel: Some(ml::Bel::Lut(ml::BelLut::A6)),
        x: ml::ExprCoord::Var("x".to_string()),
        y: ml::ExprCoord::Var("y".to_string()),
    };
    let op = ml::OpMach::Dsp(ml::OpDsp::Add);
    let opt = ml::OptMap::new();
    let dst = ml::Expr::default();
    let arg = ml::Expr::default();
    let mach = ml::InstrMach {op, opt, dst, arg, loc};
    println!("{}", ir.unwrap());
    println!("{}", asm.unwrap());
    println!("{}", tdl.unwrap());
    println!("{}", mach);
}
