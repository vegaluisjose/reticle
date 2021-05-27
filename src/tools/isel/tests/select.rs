use asm::parser::Parser as AsmParser;
use ir::parser::Parser as IrParser;
use isel::try_from_ir_prog;

#[test]
fn add() {
    let prog = IrParser::parse_from_file("../../../examples/ir/add.ir").unwrap();
    let exp = AsmParser::parse_from_file("../../../examples/asm/add.asm").unwrap();
    let res = try_from_ir_prog(&prog).unwrap();
    assert_eq!(res, exp);
}

#[test]
fn fsm_3() {
    let prog = IrParser::parse_from_file("../../../examples/ir/fsm_3.ir").unwrap();
    let exp = AsmParser::parse_from_file("../../../examples/asm/fsm_3.asm").unwrap();
    let res = try_from_ir_prog(&prog).unwrap();
    assert_eq!(res, exp);
}

#[test]
fn fsm_5() {
    let prog = IrParser::parse_from_file("../../../examples/ir/fsm_5.ir").unwrap();
    let exp = AsmParser::parse_from_file("../../../examples/asm/fsm_5.asm").unwrap();
    let res = try_from_ir_prog(&prog).unwrap();
    assert_eq!(res, exp);
}

#[test]
fn fsm_7() {
    let prog = IrParser::parse_from_file("../../../examples/ir/fsm_7.ir").unwrap();
    let exp = AsmParser::parse_from_file("../../../examples/asm/fsm_7.asm").unwrap();
    let res = try_from_ir_prog(&prog).unwrap();
    assert_eq!(res, exp);
}

#[test]
fn fsm_9() {
    let prog = IrParser::parse_from_file("../../../examples/ir/fsm_9.ir").unwrap();
    let exp = AsmParser::parse_from_file("../../../examples/asm/fsm_9.asm").unwrap();
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
