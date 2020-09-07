module register (
    input wire clock,
    input wire reset,
    input wire [7:0] a,
    input wire en,
    output wire [7:0] y
);
    wire vcc;
    wire gnd;
    VCC VCC (
        .P(vcc)
    );
    GND GND (
        .G(gnd)
    );
    FDSE # (
        .IS_D_INVERTED(1'b0),
        .INIT(1'b1),
        .IS_C_INVERTED(1'b0),
        .IS_S_INVERTED(1'b0)
    ) i0 (
        .D(a[0]),
        .Q(y[0]),
        .C(clock),
        .CE(en),
        .S(reset)
    );
    FDSE # (
        .IS_C_INVERTED(1'b0),
        .IS_S_INVERTED(1'b0),
        .INIT(1'b1),
        .IS_D_INVERTED(1'b0)
    ) i1 (
        .S(reset),
        .CE(en),
        .Q(y[1]),
        .D(a[1]),
        .C(clock)
    );
    FDRE # (
        .INIT(1'b0),
        .IS_C_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0)
    ) i2 (
        .R(reset),
        .CE(en),
        .C(clock),
        .D(a[2]),
        .Q(y[2])
    );
    FDRE # (
        .IS_C_INVERTED(1'b0),
        .INIT(1'b0),
        .IS_R_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0)
    ) i3 (
        .CE(en),
        .Q(y[3]),
        .D(a[3]),
        .R(reset),
        .C(clock)
    );
    FDRE # (
        .INIT(1'b0),
        .IS_R_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_C_INVERTED(1'b0)
    ) i4 (
        .Q(y[4]),
        .R(reset),
        .D(a[4]),
        .CE(en),
        .C(clock)
    );
    FDRE # (
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0),
        .INIT(1'b0),
        .IS_C_INVERTED(1'b0)
    ) i5 (
        .D(a[5]),
        .CE(en),
        .C(clock),
        .R(reset),
        .Q(y[5])
    );
    FDRE # (
        .IS_C_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0),
        .INIT(1'b0)
    ) i6 (
        .R(reset),
        .CE(en),
        .D(a[6]),
        .Q(y[6]),
        .C(clock)
    );
    FDRE # (
        .IS_D_INVERTED(1'b0),
        .IS_C_INVERTED(1'b0),
        .INIT(1'b0),
        .IS_R_INVERTED(1'b0)
    ) i7 (
        .C(clock),
        .CE(en),
        .R(reset),
        .D(a[7]),
        .Q(y[7])
    );
endmodule
