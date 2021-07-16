use io::read_to_string;
use std::path::Path;
use xir::parser::Parser;
use xpand::errors::Error;
use xpand::try_from_xir_prog;

fn test(name: &str) -> Result<(), Error> {
    let mut input = Path::new("../../../examples/xir").join(name);
    let mut output = Path::new("../../../examples/struct").join(name);
    input.set_extension("xir");
    output.set_extension("v");
    let parsed = Parser::parse_from_file(input)?;
    let exp = read_to_string(output);
    let res = try_from_xir_prog(&parsed, None)?;
    assert_eq!(res.to_string(), exp);
    Ok(())
}

fn test_with_mmap(name: &str) -> Result<(), Error> {
    let mut input = Path::new("../../../examples/xir").join(name);
    let mut output = Path::new("../../../examples/struct").join(name);
    let mut mem = Path::new("../../../examples/mmap").join(name);
    input.set_extension("xir");
    output.set_extension("v");
    mem.set_extension("json");
    let parsed = Parser::parse_from_file(input)?;
    let mmap = mmap::Mmap::from_file(mem);
    let exp = read_to_string(output);
    let res = try_from_xir_prog(&parsed, Some(&mmap))?;
    assert_eq!(res.to_string(), exp);
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

#[test]
fn lrom_8x8() -> Result<(), Error> {
    test_with_mmap("lrom_8x8")
}
