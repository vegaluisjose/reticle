use asm::parser::Parser as AsmParser;
use bler::assemble;
use xir::parser::Parser as XirParser;

#[test]
fn lut_add() {
    let prog = AsmParser::parse_from_file("../../../examples/asm/lut_add.asm").unwrap();
    let exp = XirParser::parse_from_file("../../../examples/xir/lut_add.xir").unwrap();
    let res = assemble(prog).unwrap();
    assert_eq!(res, exp);
}
