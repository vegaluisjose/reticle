use pretty_assertions::assert_eq;
use prim::ultrascale::bram::Bram;
use prim::ultrascale::carry::Carry;
use prim::ultrascale::gnd::Gnd;
use prim::ultrascale::lram::Lram;
use std::fmt::Display;
use xpand::dsp::Dsp;
use xpand::fdre::Fdre;
use xpand::fdse::Fdse;
use xpand::instance::ToInstance;
use xpand::lut::{Lut1, Lut2, Lut3, Lut4, Lut5, Lut6};
use xpand::to_verilog::ToVerilogInstance;
use xpand::vcc::Vcc;

fn test<S: AsRef<str>>(res: impl Display, exp: S) {
    let r = res.to_string();
    let e = exp.as_ref();
    assert_eq!(r, e);
}

#[test]
fn test_fdre() {
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
fn test_fdse() {
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
fn test_gnd() {
    let res = Gnd::default();
    let exp = r#"GND _gnd (
    .G(gnd)
);"#;
    test(res.to_instance(), exp)
}

#[test]
fn test_vcc() {
    let res = Vcc::default();
    let exp = r#"VCC _vcc (
    .P(vcc)
);"#;
    test(res.to_instance(), exp)
}

#[test]
fn test_lut1() {
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
fn test_lut2() {
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
fn test_lut3() {
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
fn test_lut4() {
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
fn test_lut5() {
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
fn test_lut6() {
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
fn test_carry() {
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
fn test_dsp() {
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

#[test]
fn test_lram() {
    let res = Lram::default();
    let exp = r#"RAM64M8 # (
    .INIT_A(64'h0000000000000000),
    .INIT_B(64'h0000000000000000),
    .INIT_C(64'h0000000000000000),
    .INIT_D(64'h0000000000000000),
    .INIT_E(64'h0000000000000000),
    .INIT_F(64'h0000000000000000),
    .INIT_G(64'h0000000000000000),
    .INIT_H(64'h0000000000000000),
    .IS_WCLK_INVERTED(1'b0)
)  (
    .ADDRA({gnd, gnd, gnd, gnd, gnd, gnd}),
    .ADDRB({gnd, gnd, gnd, gnd, gnd, gnd}),
    .ADDRC({gnd, gnd, gnd, gnd, gnd, gnd}),
    .ADDRD({gnd, gnd, gnd, gnd, gnd, gnd}),
    .ADDRE({gnd, gnd, gnd, gnd, gnd, gnd}),
    .ADDRF({gnd, gnd, gnd, gnd, gnd, gnd}),
    .ADDRG({gnd, gnd, gnd, gnd, gnd, gnd}),
    .ADDRH({gnd, gnd, gnd, gnd, gnd, gnd}),
    .DIA(gnd),
    .DIB(gnd),
    .DIC(gnd),
    .DID(gnd),
    .DIE(gnd),
    .DIF(gnd),
    .DIG(gnd),
    .DIH(gnd),
    .DOA(),
    .DOB(),
    .DOC(),
    .DOD(),
    .DOE(),
    .DOF(),
    .DOG(),
    .DOH(),
    .WCLK(gnd),
    .WE(gnd)
);"#;
    test(res.to_instance(), exp)
}

#[test]
fn test_bram() {
    let res = Bram::default();
    let exp = r#"RAMB18E2 # (
    .CASCADE_ORDER_A("NONE"),
    .CASCADE_ORDER_B("NONE"),
    .CLOCK_DOMAINS("INDEPENDENT"),
    .DOA_REG(0),
    .DOB_REG(0),
    .ENADDRENA("FALSE"),
    .ENADDRENB("FALSE"),
    .INITP_00(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INITP_01(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INITP_02(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INITP_03(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INITP_04(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INITP_05(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INITP_06(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INITP_07(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_00(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_01(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_02(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_03(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_04(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_05(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_06(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_07(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_08(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_09(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_0A(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_0B(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_0C(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_0D(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_0E(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_0F(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_10(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_11(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_12(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_13(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_14(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_15(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_16(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_17(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_18(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_19(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_1A(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_1B(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_1C(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_1D(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_1E(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_1F(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_20(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_21(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_22(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_23(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_24(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_25(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_26(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_27(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_28(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_29(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_2A(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_2B(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_2C(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_2D(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_2E(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_2F(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_30(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_31(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_32(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_33(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_34(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_35(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_36(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_37(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_38(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_39(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_3A(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_3B(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_3C(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_3D(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_3E(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_3F(256'h0000000000000000000000000000000000000000000000000000000000000000),
    .INIT_A(0),
    .INIT_B(0),
    .INIT_FILE("NONE"),
    .IS_CLKARDCLK_INVERTED(1'b0),
    .IS_CLKBWRCLK_INVERTED(1'b0),
    .IS_ENARDEN_INVERTED(1'b0),
    .IS_ENBWREN_INVERTED(1'b0),
    .IS_RSTRAMARSTRAM_INVERTED(1'b0),
    .IS_RSTRAMB_INVERTED(1'b0),
    .IS_RSTREGARSTREG_INVERTED(1'b0),
    .IS_RSTREGB_INVERTED(1'b0),
    .RDADDRCHANGEA("FALSE"),
    .RDADDRCHANGEB("FALSE"),
    .READ_WIDTH_A(0),
    .READ_WIDTH_B(0),
    .RSTREG_PRIORITY_A("RSTREG"),
    .RSTREG_PRIORITY_B("RSTREG"),
    .SIM_COLLISION_CHECK("ALL"),
    .SLEEP_ASYNC("FALSE"),
    .SRVAL_A(0),
    .SRVAL_B(0),
    .WRITE_MODE_A("NO_CHANGE"),
    .WRITE_MODE_B("NO_CHANGE"),
    .WRITE_WIDTH_A(0),
    .WRITE_WIDTH_B(0)
)  (
    .ADDRARDADDR({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
    .ADDRBWRADDR({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
    .ADDRENA(gnd),
    .ADDRENB(gnd),
    .CASDIMUXA(gnd),
    .CASDIMUXB(gnd),
    .CASDINA({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
    .CASDINB({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
    .CASDINPA({gnd, gnd}),
    .CASDINPB({gnd, gnd}),
    .CASDOMUXA(gnd),
    .CASDOMUXB(gnd),
    .CASDOMUXEN_A(gnd),
    .CASDOMUXEN_B(gnd),
    .CASDOUTA(),
    .CASDOUTB(),
    .CASDOUTPA(),
    .CASDOUTPB(),
    .CASOREGIMUXA(gnd),
    .CASOREGIMUXB(gnd),
    .CASOREGIMUXEN_A(gnd),
    .CASOREGIMUXEN_B(gnd),
    .CLKARDCLK(gnd),
    .CLKBWRCLK(gnd),
    .DINADIN({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
    .DINBDIN({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
    .DINPADINP({gnd, gnd}),
    .DINPBDINP({gnd, gnd}),
    .DOUTADOUT(),
    .DOUTBDOUT(),
    .DOUTPADOUTP(),
    .DOUTPBDOUTP(),
    .ENARDEN(gnd),
    .ENBWREN(gnd),
    .REGCEAREGCE(gnd),
    .REGCEB(gnd),
    .RSTRAMARSTRAM(gnd),
    .RSTRAMB(gnd),
    .RSTREGARSTREG(gnd),
    .RSTREGB(gnd),
    .SLEEP(gnd),
    .WEA({gnd, gnd}),
    .WEBWE({gnd, gnd, gnd, gnd})
);"#;
    test(res.to_instance(), exp)
}
