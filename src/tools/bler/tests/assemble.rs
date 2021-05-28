use asm::parser::Parser as AsmParser;
use bler::try_from_asm_prog;
use bler::errors::Error;
use std::path::Path;
use xir::parser::Parser as XirParser;

fn test(name: &str) -> Result<(), Error> {
    let mut i = Path::new("../../../examples/asm").join(name);
    let mut o = Path::new("../../../examples/xir").join(name);
    i.set_extension("asm");
    o.set_extension("xir");
    let p = AsmParser::parse_from_file(i)?;
    let exp = XirParser::parse_from_file(o)?;
    let res = try_from_asm_prog(&p)?;
    assert_eq!(res, exp);
    Ok(())
}

#[test]
fn add() -> Result<(), Error> {
    test("add")
}

#[test]
fn addreduce_placed() -> Result<(), Error> {
    test("addreduce_placed")
}

#[test]
fn fsm_3() -> Result<(), Error> {
    test("fsm_3")
}

#[test]
fn fsm_5() -> Result<(), Error> {
    test("fsm_5")
}

#[test]
fn fsm_7() -> Result<(), Error> {
    test("fsm_7")
}

#[test]
fn fsm_9() -> Result<(), Error> {
    test("fsm_9")
}
