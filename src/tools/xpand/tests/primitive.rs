use std::fmt::Display;
use xpand::errors::Error;
use xpand::fdre::Fdre;

fn test<S: AsRef<str>>(res: impl Display, exp: S) -> Result<(), Error> {
    let r = res.to_string();
    let e = exp.as_ref();
    assert_eq!(r, e, "\n\nresult:\n{}\n\nexpected:\n{}", r, e);
    Ok(())
}

#[test]
fn fdre() -> Result<(), Error> {
    let exp = r#"FDRE # (
    .INIT(1'b0),
    .IS_C_INVERTED(1'b0),
    .IS_D_INVERTED(1'b0),
    .IS_R_INVERTED(1'b0)
)  (
    .C(clock),
    .CE(gnd),
    .D(gnd),
    .Q(),
    .R(reset)
);"#;
    test(Fdre::default(), exp)
}
