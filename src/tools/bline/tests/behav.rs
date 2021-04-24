use bline::behav_try_from_ir_prog;
use io::file::read_to_string;
use ir::parser::Parser as IRParser;

#[test]
fn lut_add() {
    let prog = IRParser::parse_from_file("../../../examples/ir/lut_add.ir").unwrap();
    let exp = read_to_string("../../../examples/behav/lut_add.v");
    let res = behav_try_from_ir_prog(&prog).unwrap().to_string();
    assert_eq!(res, exp);
}