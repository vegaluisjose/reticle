use asm::parser::Parser as AsmParser;
use bler::try_from_asm_prog;
use xir::parser::Parser as XirParser;

#[test]
fn lut_add() {
    let prog = AsmParser::parse_from_file("../../../examples/asm/lut_add.asm").unwrap();
    let exp = XirParser::parse_from_file("../../../examples/xir/lut_add.xir").unwrap();
    let res = try_from_asm_prog(&prog).unwrap();
    assert_eq!(res, exp);
}
