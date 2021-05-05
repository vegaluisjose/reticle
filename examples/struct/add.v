module main (
    input wire clock,
    input wire reset,
    input wire [7:0] a,
    input wire [7:0] b,
    input wire en,
    output wire [7:0] y
);
    wire gnd;
    wire vcc;
    wire t0;
    wire t1;
    wire t2;
    wire t3;
    wire t4;
    wire t5;
    wire t6;
    wire t7;
    wire t8;
    wire t9;
    wire t10;
    wire t11;
    wire t12;
    wire t13;
    wire t14;
    wire t15;
    wire t16;
    wire t17;
    wire t18;
    wire t19;
    wire t20;
    wire t21;
    wire t22;
    wire t23;
    wire [7:0] t24;
    wire [7:0] t25;
    wire t26;
    wire t27;
    wire t28;
    wire t29;
    wire t30;
    wire t31;
    wire t32;
    wire t33;
    wire t34;
    wire t35;
    wire t36;
    wire t37;
    wire t38;
    wire t39;
    wire t40;
    wire t41;
    GND _gnd (
        .G(gnd)
    );
    VCC _vcc (
        .P(vcc)
    );
    assign t0 = a[0];
    assign t1 = a[1];
    assign t2 = a[2];
    assign t3 = a[3];
    assign t4 = a[4];
    assign t5 = a[5];
    assign t6 = a[6];
    assign t7 = a[7];
    assign t8 = b[0];
    assign t9 = b[1];
    assign t10 = b[2];
    assign t11 = b[3];
    assign t12 = b[4];
    assign t13 = b[5];
    assign t14 = b[6];
    assign t15 = b[7];
    LUT2 # (
        .INIT(4'h6)
    ) __t16 (
        .I0(t0),
        .I1(t8),
        .O(t16)
    );
    LUT2 # (
        .INIT(4'h6)
    ) __t17 (
        .I0(t1),
        .I1(t9),
        .O(t17)
    );
    LUT2 # (
        .INIT(4'h6)
    ) __t18 (
        .I0(t2),
        .I1(t10),
        .O(t18)
    );
    LUT2 # (
        .INIT(4'h6)
    ) __t19 (
        .I0(t3),
        .I1(t11),
        .O(t19)
    );
    LUT2 # (
        .INIT(4'h6)
    ) __t20 (
        .I0(t4),
        .I1(t12),
        .O(t20)
    );
    LUT2 # (
        .INIT(4'h6)
    ) __t21 (
        .I0(t5),
        .I1(t13),
        .O(t21)
    );
    LUT2 # (
        .INIT(4'h6)
    ) __t22 (
        .I0(t6),
        .I1(t14),
        .O(t22)
    );
    LUT2 # (
        .INIT(4'h6)
    ) __t23 (
        .I0(t7),
        .I1(t15),
        .O(t23)
    );
    assign t24 = {t23, t22, t21, t20, t19, t18, t17, t16};
    CARRY8 # (
        .CARRY_TYPE("SINGLE_CY8")
    ) __t25 (
        .CI(1'b0),
        .CI_TOP(1'b0),
        .CO(),
        .DI(a),
        .O(t25),
        .S(t24)
    );
    assign t26 = t25[0];
    assign t27 = t25[1];
    assign t28 = t25[2];
    assign t29 = t25[3];
    assign t30 = t25[4];
    assign t31 = t25[5];
    assign t32 = t25[6];
    assign t33 = t25[7];
    FDRE # (
        .INIT(1'b0),
        .IS_C_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0)
    ) __t34 (
        .C(clock),
        .CE(en),
        .D(t26),
        .Q(t34),
        .R(reset)
    );
    FDRE # (
        .INIT(1'b0),
        .IS_C_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0)
    ) __t35 (
        .C(clock),
        .CE(en),
        .D(t27),
        .Q(t35),
        .R(reset)
    );
    FDRE # (
        .INIT(1'b0),
        .IS_C_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0)
    ) __t36 (
        .C(clock),
        .CE(en),
        .D(t28),
        .Q(t36),
        .R(reset)
    );
    FDRE # (
        .INIT(1'b0),
        .IS_C_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0)
    ) __t37 (
        .C(clock),
        .CE(en),
        .D(t29),
        .Q(t37),
        .R(reset)
    );
    FDRE # (
        .INIT(1'b0),
        .IS_C_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0)
    ) __t38 (
        .C(clock),
        .CE(en),
        .D(t30),
        .Q(t38),
        .R(reset)
    );
    FDRE # (
        .INIT(1'b0),
        .IS_C_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0)
    ) __t39 (
        .C(clock),
        .CE(en),
        .D(t31),
        .Q(t39),
        .R(reset)
    );
    FDRE # (
        .INIT(1'b0),
        .IS_C_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0)
    ) __t40 (
        .C(clock),
        .CE(en),
        .D(t32),
        .Q(t40),
        .R(reset)
    );
    FDRE # (
        .INIT(1'b0),
        .IS_C_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0)
    ) __t41 (
        .C(clock),
        .CE(en),
        .D(t33),
        .Q(t41),
        .R(reset)
    );
    assign y = {t41, t40, t39, t38, t37, t36, t35, t34};
endmodule
