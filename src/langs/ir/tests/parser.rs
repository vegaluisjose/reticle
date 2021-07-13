use ir::ast::{Def, Prog};
use ir::parser::Parser;

#[test]
fn simple_prog() {
    let res = Parser::parse_from_str("def main () -> () {}").unwrap();
    let mut def = Def::default();
    def.set_id("main");
    let mut exp = Prog::default();
    exp.insert(&def.id(), def);
    assert_eq!(res, exp);
}
