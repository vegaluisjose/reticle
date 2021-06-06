use asm::parser::Parser as AsmParser;
use optimizer::cascader::cascader;

#[test]
fn tdot() {
    let prog = AsmParser::parse_from_file("../../../examples/asm/tdot_5_3.asm").unwrap();
    let exp = AsmParser::parse_from_file("../../../examples/asm/tdot_5_3_opt.asm").unwrap();
    let res = cascader(&prog).unwrap();
    assert_eq!(res, exp);
}
