use xpand::errors::Error;
use xpand::fdre::Fdre;
use xpand::fdse::Fdse;
use xpand::gnd::Gnd;
use xpand::instance::ToInstance;
use xpand::lut::{Lut1, Lut2, Lut3, Lut4, Lut5, Lut6};
use xpand::vcc::Vcc;

fn test<S: AsRef<str>>(res: impl ToInstance, exp: S) -> Result<(), Error> {
    let r = res.to_instance().to_string();
    let e = exp.as_ref();
    assert_eq!(r, e, "\n\nresult:\n{}\n\nexpected:\n{}", r, e);
    Ok(())
}

#[test]
fn test_fdre() -> Result<(), Error> {
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

#[test]
fn test_fdse() -> Result<(), Error> {
    let exp = r#"FDSE # (
    .INIT(1'b0),
    .IS_C_INVERTED(1'b0),
    .IS_D_INVERTED(1'b0),
    .IS_S_INVERTED(1'b0)
)  (
    .C(clock),
    .CE(gnd),
    .D(gnd),
    .Q(),
    .S(reset)
);"#;
    test(Fdse::default(), exp)
}

#[test]
fn test_gnd() -> Result<(), Error> {
    let exp = r#"GND _gnd (
    .G(gnd)
);"#;
    test(Gnd::default(), exp)
}

#[test]
fn test_vcc() -> Result<(), Error> {
    let exp = r#"VCC _vcc (
    .P(vcc)
);"#;
    test(Vcc::default(), exp)
}

#[test]
fn test_lut1() -> Result<(), Error> {
    let exp = r#"LUT1 # (
    .INIT(2'h0)
)  (
    .I0(gnd),
    .O()
);"#;
    test(Lut1::default(), exp)
}

#[test]
fn test_lut2() -> Result<(), Error> {
    let exp = r#"LUT2 # (
    .INIT(4'h0)
)  (
    .I0(gnd),
    .I1(gnd),
    .O()
);"#;
    test(Lut2::default(), exp)
}

#[test]
fn test_lut3() -> Result<(), Error> {
    let exp = r#"LUT3 # (
    .INIT(8'h0)
)  (
    .I0(gnd),
    .I1(gnd),
    .I2(gnd),
    .O()
);"#;
    test(Lut3::default(), exp)
}

#[test]
fn test_lut4() -> Result<(), Error> {
    let exp = r#"LUT4 # (
    .INIT(16'h0)
)  (
    .I0(gnd),
    .I1(gnd),
    .I2(gnd),
    .I3(gnd),
    .O()
);"#;
    test(Lut4::default(), exp)
}

#[test]
fn test_lut5() -> Result<(), Error> {
    let exp = r#"LUT5 # (
    .INIT(32'h0)
)  (
    .I0(gnd),
    .I1(gnd),
    .I2(gnd),
    .I3(gnd),
    .I4(gnd),
    .O()
);"#;
    test(Lut5::default(), exp)
}

#[test]
fn test_lut6() -> Result<(), Error> {
    let exp = r#"LUT6 # (
    .INIT(64'h0)
)  (
    .I0(gnd),
    .I1(gnd),
    .I2(gnd),
    .I3(gnd),
    .I4(gnd),
    .I5(gnd),
    .O()
);"#;
    test(Lut6::default(), exp)
}
