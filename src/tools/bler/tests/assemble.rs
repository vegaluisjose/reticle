use asm::parser::Parser as AsmParser;
use bler::try_from_asm_prog;
use std::path::Path;
use xir::parser::Parser as XirParser;

fn test(name: &str) {
    let mut i = Path::new("../../../examples/asm").join(name);
    let mut o = Path::new("../../../examples/xir").join(name);
    i.set_extension("asm");
    o.set_extension("xir");
    let p = AsmParser::parse_from_file(i).unwrap();
    let exp = XirParser::parse_from_file(o).unwrap();
    let res = try_from_asm_prog(&p).unwrap();
    assert_eq!(res, exp);
}

#[test]
fn add() {
    test("add");
}

#[test]
fn addreduce_placed() {
    test("addreduce_placed");
}

#[test]
fn fsm_3() {
    test("fsm_3");
}

#[test]
fn fsm_5() {
    test("fsm_5");
}

#[test]
fn fsm_7() {
    test("fsm_7");
}

#[test]
fn fsm_9() {
    test("fsm_9");
}
