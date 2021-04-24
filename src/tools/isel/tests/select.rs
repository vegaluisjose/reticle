use asm::parser::Parser as AsmParser;
use ir::parser::Parser as IrParser;
use isel::try_from_ir_prog;

#[test]
fn lut_add() {
    let prog = IrParser::parse_from_file("../../../examples/ir/lut_add.ir").unwrap();
    let exp = AsmParser::parse_from_file("../../../examples/asm/lut_add.asm").unwrap();
    let res = try_from_ir_prog(&prog).unwrap();
    assert_eq!(res, exp);
}

#[test]
fn fsm() {
    let prog = IrParser::parse_from_file("../../../examples/ir/fsm.ir").unwrap();
    let exp = AsmParser::parse_from_file("../../../examples/asm/fsm.asm").unwrap();
    let res = try_from_ir_prog(&prog).unwrap();
    assert_eq!(res, exp);
}

#[test]
fn tadd() {
    let prog = IrParser::parse_from_file("../../../examples/ir/tadd.ir").unwrap();
    let exp = AsmParser::parse_from_file("../../../examples/asm/tadd.asm").unwrap();
    let res = try_from_ir_prog(&prog).unwrap();
    assert_eq!(res, exp);
}

#[test]
fn tdot() {
    let prog = IrParser::parse_from_file("../../../examples/ir/tdot.ir").unwrap();
    let exp = AsmParser::parse_from_file("../../../examples/asm/tdot.asm").unwrap();
    let res = try_from_ir_prog(&prog).unwrap();
    assert_eq!(res, exp);
}
