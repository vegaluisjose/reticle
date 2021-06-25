use std::fmt::Display;
use xpand::carry::Carry;
use xpand::dsp::Dsp;
use xpand::errors::Error;
use xpand::fdre::Fdre;
use xpand::fdse::Fdse;
use xpand::gnd::Gnd;
use xpand::instance::ToInstance;
use xpand::lut::{Lut1, Lut2, Lut3, Lut4, Lut5, Lut6};
use xpand::vcc::Vcc;

fn test<S: AsRef<str>>(res: impl Display, exp: S) -> Result<(), Error> {
    let r = res.to_string();
    let e = exp.as_ref();
    assert_eq!(r, e, "\n\nresult:\n{}\n\nexpected:\n{}", r, e);
    Ok(())
}

#[test]
fn test_fdre() -> Result<(), Error> {
    let res = Fdre::default();
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
    test(res.to_instance(), exp)
}

#[test]
fn test_fdse() -> Result<(), Error> {
    let res = Fdse::default();
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
    test(res.to_instance(), exp)
}

#[test]
fn test_gnd() -> Result<(), Error> {
    let res = Gnd::default();
    let exp = r#"GND _gnd (
    .G(gnd)
);"#;
    test(res.to_instance(), exp)
}

#[test]
fn test_vcc() -> Result<(), Error> {
    let res = Vcc::default();
    let exp = r#"VCC _vcc (
    .P(vcc)
);"#;
    test(res.to_instance(), exp)
}

#[test]
fn test_lut1() -> Result<(), Error> {
    let res = Lut1::default();
    let exp = r#"LUT1 # (
    .INIT(2'h0)
)  (
    .I0(gnd),
    .O()
);"#;
    test(res.to_instance(), exp)
}

#[test]
fn test_lut2() -> Result<(), Error> {
    let res = Lut2::default();
    let exp = r#"LUT2 # (
    .INIT(4'h0)
)  (
    .I0(gnd),
    .I1(gnd),
    .O()
);"#;
    test(res.to_instance(), exp)
}

#[test]
fn test_lut3() -> Result<(), Error> {
    let res = Lut3::default();
    let exp = r#"LUT3 # (
    .INIT(8'h0)
)  (
    .I0(gnd),
    .I1(gnd),
    .I2(gnd),
    .O()
);"#;
    test(res.to_instance(), exp)
}

#[test]
fn test_lut4() -> Result<(), Error> {
    let res = Lut4::default();
    let exp = r#"LUT4 # (
    .INIT(16'h0)
)  (
    .I0(gnd),
    .I1(gnd),
    .I2(gnd),
    .I3(gnd),
    .O()
);"#;
    test(res.to_instance(), exp)
}

#[test]
fn test_lut5() -> Result<(), Error> {
    let res = Lut5::default();
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
    test(res.to_instance(), exp)
}

#[test]
fn test_lut6() -> Result<(), Error> {
    let res = Lut6::default();
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
    test(res.to_instance(), exp)
}

#[test]
fn test_carry() -> Result<(), Error> {
    let res = Carry::default();
    let exp = r#"CARRY8 # (
    .CARRY_TYPE("SINGLE_CY8")
)  (
    .CI(gnd),
    .CI_TOP(gnd),
    .CO(),
    .DI({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
    .O(),
    .S({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd})
);"#;
    test(res.to_instance(), exp)
}

#[test]
fn test_dsp() -> Result<(), Error> {
    let res = Dsp::default();
    let exp = r#"DSP48E2 # (
    .ACASCREG(0),
    .ADREG(0),
    .ALUMODEREG(0),
    .AMULTSEL("A"),
    .AREG(0),
    .AUTORESET_PATDET("NO_RESET"),
    .AUTORESET_PRIORITY("RESET"),
    .A_INPUT("DIRECT"),
    .BCASCREG(0),
    .BMULTSEL("B"),
    .BREG(0),
    .B_INPUT("DIRECT"),
    .CARRYINREG(0),
    .CARRYINSELREG(0),
    .CREG(0),
    .DREG(0),
    .INMODEREG(0),
    .IS_ALUMODE_INVERTED(4'h0),
    .IS_CARRYIN_INVERTED(1'b0),
    .IS_CLK_INVERTED(1'b0),
    .IS_INMODE_INVERTED(5'h0),
    .IS_OPMODE_INVERTED(9'h0),
    .IS_RSTALLCARRYIN_INVERTED(1'b0),
    .IS_RSTALUMODE_INVERTED(1'b0),
    .IS_RSTA_INVERTED(1'b0),
    .IS_RSTB_INVERTED(1'b0),
    .IS_RSTCTRL_INVERTED(1'b0),
    .IS_RSTC_INVERTED(1'b0),
    .IS_RSTD_INVERTED(1'b0),
    .IS_RSTINMODE_INVERTED(1'b0),
    .IS_RSTM_INVERTED(1'b0),
    .IS_RSTP_INVERTED(1'b0),
    .MASK(48'h3fffffffffff),
    .MREG(0),
    .OPMODEREG(0),
    .PATTERN(48'h0),
    .PREADDINSEL("A"),
    .PREG(0),
    .RND(48'h0),
    .SEL_MASK("MASK"),
    .SEL_PATTERN("PATTERN"),
    .USE_MULT("NONE"),
    .USE_SIMD("ONE48"),
    .USE_WIDEXOR("FALSE"),
    .XORSIMD("XOR24_48_96")
)  (
    .A({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
    .ACIN({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
    .ACOUT(),
    .ALUMODE({gnd, gnd, gnd, gnd}),
    .B({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
    .BCIN({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
    .BCOUT(),
    .C({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
    .CARRYCASCIN(gnd),
    .CARRYCASCOUT(),
    .CARRYIN(gnd),
    .CARRYINSEL({gnd, gnd, gnd}),
    .CARRYOUT(),
    .CEA1(gnd),
    .CEA2(gnd),
    .CEAD(gnd),
    .CEALUMODE(gnd),
    .CEB1(gnd),
    .CEB2(gnd),
    .CEC(gnd),
    .CECARRYIN(gnd),
    .CECTRL(gnd),
    .CED(gnd),
    .CEINMODE(gnd),
    .CEM(gnd),
    .CEP(gnd),
    .CLK(clock),
    .D({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
    .INMODE({gnd, gnd, gnd, gnd, gnd}),
    .MULTSIGNIN(gnd),
    .MULTSIGNOUT(),
    .OPMODE({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
    .OVERFLOW(),
    .P(),
    .PATTERNBDETECT(),
    .PATTERNDETECT(),
    .PCIN({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
    .PCOUT(),
    .RSTA(reset),
    .RSTALLCARRYIN(reset),
    .RSTALUMODE(reset),
    .RSTB(reset),
    .RSTC(reset),
    .RSTCTRL(reset),
    .RSTD(reset),
    .RSTINMODE(reset),
    .RSTM(reset),
    .RSTP(reset),
    .UNDERFLOW(),
    .XOROUT()
);"#;
    test(res.to_instance(), exp)
}
