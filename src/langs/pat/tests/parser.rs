use pat::ast::*;
use pat::parser::Parser;

#[test]
fn simple_target() {
    let res = Parser::parse_from_str("pat add_i8_0 () -> () {}").unwrap();
    let mut sig = Sig::default();
    sig.set_id("add_i8_0");
    let mut pat = Pat::default();
    pat.set_sig(sig);
    let mut exp = Target::default();
    exp.insert(&pat.id(), pat);
    assert_eq!(res, exp);
}
