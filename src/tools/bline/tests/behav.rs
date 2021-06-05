use bline::try_from_ir_prog;
use io::file::read_to_string;
use ir::parser::Parser as IrParser;

#[test]
fn add() {
    let prog = IrParser::parse_from_file("../../../examples/ir/add.ir").unwrap();
    let exp = read_to_string("../../../examples/behav/add.v");
    let res = try_from_ir_prog(&prog).unwrap().to_string();
    assert_eq!(res, exp);
}
