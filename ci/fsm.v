module fsm (
    input wire clock,
    input wire reset,
    input wire start,
    input wire done,
    output wire [7:0] state
);
    wire vcc;
    wire gnd;
    wire [7:0] t0;
    wire [7:0] t1;
    wire [7:0] t2;
    wire [7:0] t6;
    wire t7;
    wire t8;
    wire t9;
    wire t10;
    wire t11;
    wire t12;
    wire t3;
    wire t13;
    wire [7:0] t4;
    wire t5;
    VCC VCC (
        .P(vcc)
    );
    GND GND (
        .G(gnd)
    );
    FDRE # (
        .IS_R_INVERTED(1'b0),
        .IS_C_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .INIT(1'b0)
    ) i1 (
        .R(reset),
        .C(clock),
        .CE(t5),
        .D(t6[0]),
        .Q(t2[0])
    );
    LUT3 # (
        .INIT(8'hac)
    ) i0 (
        .I0(t3),
        .I2(t4[0]),
        .I1(t1[0]),
        .O(t6[0])
    );
    FDRE # (
        .INIT(1'b0),
        .IS_C_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0)
    ) i3 (
        .R(reset),
        .CE(t5),
        .C(clock),
        .D(t6[1]),
        .Q(t2[1])
    );
    LUT3 # (
        .INIT(8'hac)
    ) i2 (
        .I1(t1[1]),
        .O(t6[1]),
        .I2(t4[1]),
        .I0(t3)
    );
    FDRE # (
        .IS_R_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_C_INVERTED(1'b0),
        .INIT(1'b0)
    ) i5 (
        .D(t6[2]),
        .Q(t2[2]),
        .CE(t5),
        .R(reset),
        .C(clock)
    );
    LUT3 # (
        .INIT(8'hac)
    ) i4 (
        .I1(t1[2]),
        .O(t6[2]),
        .I2(t4[2]),
        .I0(t3)
    );
    FDRE # (
        .IS_R_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_C_INVERTED(1'b0),
        .INIT(1'b0)
    ) i7 (
        .CE(t5),
        .C(clock),
        .Q(t2[3]),
        .R(reset),
        .D(t6[3])
    );
    LUT3 # (
        .INIT(8'hac)
    ) i6 (
        .I2(t4[3]),
        .I0(t3),
        .I1(t1[3]),
        .O(t6[3])
    );
    FDRE # (
        .IS_C_INVERTED(1'b0),
        .INIT(1'b0),
        .IS_R_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0)
    ) i9 (
        .Q(t2[4]),
        .C(clock),
        .R(reset),
        .D(t6[4]),
        .CE(t5)
    );
    LUT3 # (
        .INIT(8'hac)
    ) i8 (
        .I2(t4[4]),
        .O(t6[4]),
        .I0(t3),
        .I1(t1[4])
    );
    FDRE # (
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0),
        .IS_C_INVERTED(1'b0),
        .INIT(1'b0)
    ) i11 (
        .C(clock),
        .R(reset),
        .D(t6[5]),
        .CE(t5),
        .Q(t2[5])
    );
    LUT3 # (
        .INIT(8'hac)
    ) i10 (
        .I1(t1[5]),
        .I2(t4[5]),
        .I0(t3),
        .O(t6[5])
    );
    FDRE # (
        .IS_C_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0),
        .INIT(1'b0),
        .IS_D_INVERTED(1'b0)
    ) i13 (
        .CE(t5),
        .D(t6[6]),
        .Q(t2[6]),
        .C(clock),
        .R(reset)
    );
    LUT3 # (
        .INIT(8'hac)
    ) i12 (
        .I1(t1[6]),
        .O(t6[6]),
        .I0(t3),
        .I2(t4[6])
    );
    FDRE # (
        .INIT(1'b0),
        .IS_R_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_C_INVERTED(1'b0)
    ) i15 (
        .D(t6[7]),
        .R(reset),
        .CE(t5),
        .Q(t2[7]),
        .C(clock)
    );
    LUT3 # (
        .INIT(8'hac)
    ) i14 (
        .I2(t4[7]),
        .O(t6[7]),
        .I1(t1[7]),
        .I0(t3)
    );
    LUT6 # (
        .INIT(64'h9009000000000000)
    ) i16 (
        .O(t7),
        .I3(t2[6]),
        .I0(t0[7]),
        .I1(t2[7]),
        .I5(t9),
        .I4(t8),
        .I2(t0[6])
    );
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) i17 (
        .I1(t0[3]),
        .I0(t2[3]),
        .I3(t2[5]),
        .I4(t0[4]),
        .I2(t0[5]),
        .I5(t2[4]),
        .O(t8)
    );
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) i18 (
        .I0(t2[0]),
        .I2(t0[2]),
        .I4(t0[1]),
        .I5(t2[1]),
        .I1(t0[0]),
        .O(t9),
        .I3(t2[2])
    );
    LUT6 # (
        .INIT(64'h9009000000000000)
    ) i19 (
        .I0(t1[7]),
        .I5(t12),
        .I2(t1[6]),
        .I4(t11),
        .I1(t2[7]),
        .I3(t2[6]),
        .O(t10)
    );
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) i20 (
        .I0(t2[3]),
        .I1(t1[3]),
        .I4(t1[4]),
        .I5(t2[4]),
        .O(t11),
        .I2(t1[5]),
        .I3(t2[5])
    );
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) i21 (
        .I4(t1[1]),
        .I2(t1[2]),
        .O(t12),
        .I0(t2[0]),
        .I1(t1[0]),
        .I3(t2[2]),
        .I5(t2[1])
    );
    LUT2 # (
        .INIT(4'h8)
    ) i22 (
        .O(t3),
        .I0(t7),
        .I1(start)
    );
    LUT2 # (
        .INIT(4'h8)
    ) i23 (
        .O(t13),
        .I0(t10),
        .I1(done)
    );
    LUT3 # (
        .INIT(8'hac)
    ) i24 (
        .I1(t0[0]),
        .O(t4[0]),
        .I2(t2[0]),
        .I0(t13)
    );
    LUT3 # (
        .INIT(8'hac)
    ) i25 (
        .I0(t13),
        .O(t4[1]),
        .I1(t0[1]),
        .I2(t2[1])
    );
    LUT3 # (
        .INIT(8'hac)
    ) i26 (
        .I0(t13),
        .O(t4[2]),
        .I1(t0[2]),
        .I2(t2[2])
    );
    LUT3 # (
        .INIT(8'hac)
    ) i27 (
        .I1(t0[3]),
        .O(t4[3]),
        .I0(t13),
        .I2(t2[3])
    );
    LUT3 # (
        .INIT(8'hac)
    ) i28 (
        .I1(t0[4]),
        .O(t4[4]),
        .I0(t13),
        .I2(t2[4])
    );
    LUT3 # (
        .INIT(8'hac)
    ) i29 (
        .I1(t0[5]),
        .I0(t13),
        .I2(t2[5]),
        .O(t4[5])
    );
    LUT3 # (
        .INIT(8'hac)
    ) i30 (
        .I1(t0[6]),
        .I2(t2[6]),
        .O(t4[6]),
        .I0(t13)
    );
    LUT3 # (
        .INIT(8'hac)
    ) i31 (
        .I1(t0[7]),
        .I2(t2[7]),
        .I0(t13),
        .O(t4[7])
    );
    LUT2 # (
        .INIT(4'he)
    ) i32 (
        .O(t5),
        .I1(t13),
        .I0(t3)
    );
    assign t0 = {gnd, gnd, gnd, gnd, gnd, gnd, gnd, gnd};
    assign t1 = {gnd, gnd, gnd, gnd, gnd, gnd, gnd, vcc};
    assign state = t2;
endmodule
