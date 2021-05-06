use io::file::read_to_string;
use xir::parser::Parser;
use xpand::try_from_xir_prog;

#[test]
fn add() {
    let prog = Parser::parse_from_file("../../../examples/xir/add.xir").unwrap();
    let exp = read_to_string("../../../examples/struct/add.v");
    let res = try_from_xir_prog(&prog).unwrap().to_string();
    assert_eq!(res, exp);
}
