use asm::parser::Parser as AsmParser;
use ir::parser::Parser as IrParser;
use isel::errors::Error;
use isel::try_from_ir_prog;
use std::path::Path;

fn test(name: &str) -> Result<(), Error> {
    let mut i = Path::new("../../../examples/ir").join(name);
    let mut o = Path::new("../../../examples/asm").join(name);
    i.set_extension("ir");
    o.set_extension("asm");
    let p = IrParser::parse_from_file(i)?;
    let e = AsmParser::parse_from_file(o)?;
    let r = try_from_ir_prog(&p)?;
    assert_eq!(r, e);
    Ok(())
}

#[test]
fn add() -> Result<(), Error> {
    test("add")
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

#[test]
fn tadd() -> Result<(), Error> {
    test("tadd")
}

#[test]
fn tadd_64() -> Result<(), Error> {
    test("tadd_64")
}

#[test]
fn tadd_128() -> Result<(), Error> {
    test("tadd_128")
}

#[test]
fn tadd_256() -> Result<(), Error> {
    test("tadd_256")
}

#[test]
fn tadd_512() -> Result<(), Error> {
    test("tadd_512")
}

#[test]
fn tdot() -> Result<(), Error> {
    test("tdot")
}

#[test]
fn tdot_5_3() -> Result<(), Error> {
    test("tdot_5_3")
}

#[test]
fn tdot_5_9() -> Result<(), Error> {
    test("tdot_5_9")
}

#[test]
fn tdot_5_18() -> Result<(), Error> {
    test("tdot_5_18")
}

#[test]
fn tdot_5_36() -> Result<(), Error> {
    test("tdot_5_36")
}
