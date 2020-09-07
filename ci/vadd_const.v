module vadd_const (
    input wire clock,
    input wire reset,
    input wire [7:0] a_0,
    input wire [7:0] a_1,
    input wire [7:0] a_2,
    input wire [7:0] a_3,
    output wire [7:0] y_0,
    output wire [7:0] y_1,
    output wire [7:0] y_2,
    output wire [7:0] y_3
);
    wire vcc;
    wire gnd;
    wire [7:0] t0;
    wire [7:0] t1;
    wire [7:0] t2;
    wire [7:0] t3;
    wire [47:0] t4;
    wire [47:0] t5;
    wire [47:0] t6;
    VCC VCC (
        .P(vcc)
    );
    GND GND (
        .G(gnd)
    );
    DSP48E2 # (
        .IS_RSTB_INVERTED(1'b0),
        .AMULTSEL("A"),
        .IS_RSTCTRL_INVERTED(1'b0),
        .ADREG(0),
        .IS_RSTC_INVERTED(1'b0),
        .USE_MULT("NONE"),
        .IS_ALUMODE_INVERTED(4'b0000),
        .PREADDINSEL("A"),
        .B_INPUT("DIRECT"),
        .AUTORESET_PATDET("NO_RESET"),
        .INMODEREG(0),
        .CARRYINREG(0),
        .IS_OPMODE_INVERTED(9'b000000000),
        .BREG(0),
        .BCASCREG(0),
        .PATTERN(48'h0),
        .IS_RSTALUMODE_INVERTED(1'b0),
        .SEL_MASK("MASK"),
        .ACASCREG(0),
        .RND(48'h0),
        .XORSIMD("XOR24_48_96"),
        .USE_WIDEXOR("FALSE"),
        .IS_CARRYIN_INVERTED(1'b0),
        .IS_RSTINMODE_INVERTED(1'b0),
        .CREG(0),
        .IS_RSTM_INVERTED(1'b0),
        .IS_RSTP_INVERTED(1'b0),
        .MREG(0),
        .SEL_PATTERN("PATTERN"),
        .ALUMODEREG(0),
        .USE_SIMD("FOUR12"),
        .MASK(48'h3fffffffffff),
        .USE_PATTERN_DETECT("NO_PATDET"),
        .IS_CLK_INVERTED(1'b0),
        .IS_RSTALLCARRYIN_INVERTED(1'b0),
        .IS_RSTD_INVERTED(1'b0),
        .AREG(0),
        .CARRYINSELREG(0),
        .IS_RSTA_INVERTED(1'b0),
        .A_INPUT("DIRECT"),
        .DREG(0),
        .OPMODEREG(0),
        .PREG(0),
        .AUTORESET_PRIORITY("RESET"),
        .IS_INMODE_INVERTED(5'b00000),
        .BMULTSEL("B")
    ) i0 (
        .CARRYIN(1'b0),
        .RSTC(reset),
        .CECTRL(1'b0),
        .MULTSIGNIN(1'b0),
        .B(t5[17:0]),
        .RSTINMODE(reset),
        .CLK(clock),
        .CEINMODE(1'b0),
        .RSTD(reset),
        .CEA2(1'b0),
        .CEA1(1'b0),
        .PATTERNBDETECT(),
        .RSTP(reset),
        .XOROUT(),
        .CED(1'b0),
        .RSTALLCARRYIN(reset),
        .INMODE(5'b00000),
        .P(t6),
        .CEALUMODE(1'b0),
        .OVERFLOW(),
        .CECARRYIN(1'b0),
        .ACIN(30'd0),
        .CARRYCASCIN(1'b0),
        .CEM(1'b0),
        .CARRYINSEL(3'd0),
        .CARRYCASCOUT(),
        .RSTM(reset),
        .RSTB(reset),
        .ACOUT(),
        .C(t4),
        .CEC(1'b0),
        .MULTSIGNOUT(),
        .A(t5[47:18]),
        .OPMODE(9'b000110011),
        .UNDERFLOW(),
        .CEB2(1'b0),
        .CEB1(1'b0),
        .CEP(1'b0),
        .BCIN(18'd0),
        .PCIN(18'd0),
        .D(18'd0),
        .PATTERNDETECT(),
        .RSTCTRL(reset),
        .RSTALUMODE(reset),
        .ALUMODE(4'b0000),
        .RSTA(reset),
        .CEAD(1'b0),
        .PCOUT(),
        .CARRYOUT(),
        .BCOUT()
    );
    assign t0 = {gnd, gnd, gnd, gnd, gnd, gnd, vcc, gnd};
    assign t1 = {vcc, vcc, vcc, vcc, vcc, vcc, gnd, gnd};
    assign t2 = {gnd, gnd, gnd, gnd, gnd, vcc, gnd, vcc};
    assign t3 = {vcc, vcc, vcc, vcc, vcc, vcc, gnd, vcc};
    assign t4 = {gnd, gnd, gnd, gnd, t3, gnd, gnd, gnd, gnd, t2, gnd, gnd, gnd, gnd, t1, gnd, gnd, gnd, gnd, t0};
    assign t5 = {gnd, gnd, gnd, gnd, a_3, gnd, gnd, gnd, gnd, a_2, gnd, gnd, gnd, gnd, a_1, gnd, gnd, gnd, gnd, a_0};
    assign y_0 = t6[7:0];
    assign y_1 = t6[19:12];
    assign y_2 = t6[31:24];
    assign y_3 = t6[43:36];
endmodule
