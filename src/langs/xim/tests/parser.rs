use xim::ast::*;
use xim::parser::Parser;

#[test]
fn simple_target() {
    let res = Parser::parse_from_str("imp add_i8_0[1, 1]() -> () {}").unwrap();
    let mut sig = Sig::default();
    sig.set_id("add_i8_0");
    sig.set_area(1);
    sig.set_perf(1);
    let mut imp = Imp::default();
    imp.set_sig(sig);
    let mut exp = Target::default();
    exp.insert(&imp.id(), imp);
    assert_eq!(res, exp);
}
