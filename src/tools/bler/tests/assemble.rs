use asm::parser::Parser as AsmParser;
use bler::try_from_asm_prog;
use xir::parser::Parser as XirParser;

#[test]
fn add() {
    let prog = AsmParser::parse_from_file("../../../examples/asm/add.asm").unwrap();
    let exp = XirParser::parse_from_file("../../../examples/xir/add.xir").unwrap();
    let res = try_from_asm_prog(&prog).unwrap();
    assert_eq!(res, exp);
}

#[test]
fn addreduce_placed() {
    let prog = AsmParser::parse_from_file("../../../examples/asm/addreduce_placed.asm").unwrap();
    let exp = XirParser::parse_from_file("../../../examples/xir/addreduce_placed.xir").unwrap();
    let res = try_from_asm_prog(&prog).unwrap();
    assert_eq!(res, exp);
}
