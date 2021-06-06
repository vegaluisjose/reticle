use io::file::read_to_string;
use std::path::Path;
use xir::parser::Parser;
use xpand::errors::Error;
use xpand::try_from_xir_prog;

fn test(name: &str) -> Result<(), Error> {
    let mut i = Path::new("../../../examples/xir").join(name);
    let mut o = Path::new("../../../examples/struct").join(name);
    i.set_extension("xir");
    o.set_extension("v");
    let p = Parser::parse_from_file(i)?;
    let e = read_to_string(o);
    let r = try_from_xir_prog(&p)?;
    assert_eq!(r.to_string(), e);
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

#[test]
fn tadd() -> Result<(), Error> {
    test("tadd")
}
