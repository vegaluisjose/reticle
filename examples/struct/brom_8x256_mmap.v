module main (
    input wire clock,
    input wire reset,
    input wire [7:0] addr,
    output wire [7:0] y
);
    wire gnd;
    wire vcc;
    GND _gnd (
        .G(gnd)
    );
    VCC _vcc (
        .P(vcc)
    );
    (*LOC = "SLICE_X0Y0", BEL = "RAMB18E2_U"*)
    RAMB18E2 # (
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
        .INIT_00(256'h0000000000000000000000000000000000000000000000002211FFEEDDCCBBAA),
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
        .INIT_A(18'h0),
        .INIT_B(18'h0),
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
        .READ_WIDTH_A(9),
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
    ) __y (
        .ADDRARDADDR({gnd, gnd, gnd, addr, gnd, gnd, gnd}),
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
        .CLKARDCLK(clock),
        .CLKBWRCLK(gnd),
        .DINADIN({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
        .DINBDIN({gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd}),
        .DINPADINP({gnd, gnd}),
        .DINPBDINP({gnd, gnd}),
        .DOUTADOUT(y),
        .DOUTBDOUT(),
        .DOUTPADOUTP(),
        .DOUTPBDOUTP(),
        .ENARDEN(vcc),
        .ENBWREN(gnd),
        .REGCEAREGCE(gnd),
        .REGCEB(gnd),
        .RSTRAMARSTRAM(reset),
        .RSTRAMB(gnd),
        .RSTREGARSTREG(reset),
        .RSTREGB(gnd),
        .SLEEP(gnd),
        .WEA({gnd, gnd}),
        .WEBWE({gnd, gnd, gnd, gnd})
    );
endmodule
