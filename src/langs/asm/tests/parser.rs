use asm::ast::Prog;
use asm::parser::Parser;

#[test]
fn simple_prog() {
    let res = Parser::parse_from_str("def main () -> () {}").unwrap();
    let mut exp = Prog::default();
    exp.set_id("main");
    assert_eq!(res, exp);
}
