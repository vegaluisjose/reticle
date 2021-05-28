module main (
    input wire clock,
    input wire reset,
    input wire i0,
    input wire i1,
    input wire i2,
    output wire [3:0] y
);
    wire gnd;
    wire vcc;
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
    wire [3:0] t4;
    wire t3;
    wire t0;
    wire t22;
    wire t23;
    wire t24;
    wire t25;
    wire [3:0] t1;
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
    wire [3:0] t2;
    wire t41;
    wire t42;
    wire t43;
    wire t44;
    wire t45;
    wire t46;
    wire t47;
    wire t48;
    wire t49;
    wire t21;
    wire t26;
    wire t51;
    wire t52;
    wire t53;
    wire t54;
    wire [3:0] t27;
    wire t57;
    wire t58;
    wire t59;
    wire t60;
    wire t61;
    wire t62;
    wire t63;
    wire t64;
    wire t65;
    wire t66;
    wire t67;
    wire t68;
    wire [3:0] t28;
    wire t69;
    wire t70;
    wire t71;
    wire t72;
    wire t73;
    wire t74;
    wire t75;
    wire t76;
    wire t77;
    wire t50;
    wire t55;
    wire t79;
    wire t80;
    wire t81;
    wire t82;
    wire [3:0] t56;
    wire t83;
    wire t84;
    wire t85;
    wire t86;
    wire t87;
    wire t88;
    wire t89;
    wire t90;
    wire t91;
    wire t78;
    GND _gnd (
        .G(gnd)
    );
    VCC _vcc (
        .P(vcc)
    );
    assign t5 = t1[0];
    assign t6 = t2[0];
    assign t7 = t1[1];
    assign t8 = t2[1];
    assign t9 = t1[2];
    assign t10 = t2[2];
    assign t11 = t1[3];
    assign t12 = t2[3];
    LUT3 # (
        .INIT(8'hAC)
    ) __t13 (
        .I0(t5),
        .I1(t6),
        .I2(t0),
        .O(t13)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t14 (
        .I0(t7),
        .I1(t8),
        .I2(t0),
        .O(t14)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t15 (
        .I0(t9),
        .I1(t10),
        .I2(t0),
        .O(t15)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t16 (
        .I0(t11),
        .I1(t12),
        .I2(t0),
        .O(t16)
    );
    FDRE # (
        .INIT(1'b0),
        .IS_C_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0)
    ) __t17 (
        .C(clock),
        .CE(t3),
        .D(t13),
        .Q(t17),
        .R(reset)
    );
    FDRE # (
        .INIT(1'b0),
        .IS_C_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0)
    ) __t18 (
        .C(clock),
        .CE(t3),
        .D(t14),
        .Q(t18),
        .R(reset)
    );
    FDRE # (
        .INIT(1'b0),
        .IS_C_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0)
    ) __t19 (
        .C(clock),
        .CE(t3),
        .D(t15),
        .Q(t19),
        .R(reset)
    );
    FDRE # (
        .INIT(1'b0),
        .IS_C_INVERTED(1'b0),
        .IS_D_INVERTED(1'b0),
        .IS_R_INVERTED(1'b0)
    ) __t20 (
        .C(clock),
        .CE(t3),
        .D(t16),
        .Q(t20),
        .R(reset)
    );
    assign t4 = {t20, t19, t18, t17};
    assign t3 = vcc;
    LUT2 # (
        .INIT(4'h8)
    ) __t0 (
        .I0(t21),
        .I1(i2),
        .O(t0)
    );
    assign t22 = gnd;
    assign t23 = gnd;
    assign t24 = gnd;
    assign t25 = gnd;
    assign t1 = {t25, t24, t23, t22};
    assign t29 = t27[0];
    assign t30 = t28[0];
    assign t31 = t27[1];
    assign t32 = t28[1];
    assign t33 = t27[2];
    assign t34 = t28[2];
    assign t35 = t27[3];
    assign t36 = t28[3];
    LUT3 # (
        .INIT(8'hAC)
    ) __t37 (
        .I0(t29),
        .I1(t30),
        .I2(t26),
        .O(t37)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t38 (
        .I0(t31),
        .I1(t32),
        .I2(t26),
        .O(t38)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t39 (
        .I0(t33),
        .I1(t34),
        .I2(t26),
        .O(t39)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t40 (
        .I0(t35),
        .I1(t36),
        .I2(t26),
        .O(t40)
    );
    assign t2 = {t40, t39, t38, t37};
    assign t41 = t4[0];
    assign t42 = t27[0];
    assign t43 = t4[1];
    assign t44 = t27[1];
    assign t45 = t4[2];
    assign t46 = t27[2];
    assign t47 = t4[3];
    assign t48 = t27[3];
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) __t49 (
        .I0(t41),
        .I1(t42),
        .I2(t46),
        .I3(t45),
        .I4(t44),
        .I5(t43),
        .O(t49)
    );
    LUT3 # (
        .INIT(8'h90)
    ) __t21 (
        .I0(t47),
        .I1(t48),
        .I2(t49),
        .O(t21)
    );
    LUT2 # (
        .INIT(4'h8)
    ) __t26 (
        .I0(t50),
        .I1(i1),
        .O(t26)
    );
    assign t51 = gnd;
    assign t52 = vcc;
    assign t53 = gnd;
    assign t54 = gnd;
    assign t27 = {t54, t53, t52, t51};
    assign t57 = t56[0];
    assign t58 = t4[0];
    assign t59 = t56[1];
    assign t60 = t4[1];
    assign t61 = t56[2];
    assign t62 = t4[2];
    assign t63 = t56[3];
    assign t64 = t4[3];
    LUT3 # (
        .INIT(8'hAC)
    ) __t65 (
        .I0(t57),
        .I1(t58),
        .I2(t55),
        .O(t65)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t66 (
        .I0(t59),
        .I1(t60),
        .I2(t55),
        .O(t66)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t67 (
        .I0(t61),
        .I1(t62),
        .I2(t55),
        .O(t67)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t68 (
        .I0(t63),
        .I1(t64),
        .I2(t55),
        .O(t68)
    );
    assign t28 = {t68, t67, t66, t65};
    assign t69 = t4[0];
    assign t70 = t56[0];
    assign t71 = t4[1];
    assign t72 = t56[1];
    assign t73 = t4[2];
    assign t74 = t56[2];
    assign t75 = t4[3];
    assign t76 = t56[3];
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) __t77 (
        .I0(t69),
        .I1(t70),
        .I2(t74),
        .I3(t73),
        .I4(t72),
        .I5(t71),
        .O(t77)
    );
    LUT3 # (
        .INIT(8'h90)
    ) __t50 (
        .I0(t75),
        .I1(t76),
        .I2(t77),
        .O(t50)
    );
    LUT2 # (
        .INIT(4'h8)
    ) __t55 (
        .I0(t78),
        .I1(i0),
        .O(t55)
    );
    assign t79 = vcc;
    assign t80 = gnd;
    assign t81 = gnd;
    assign t82 = gnd;
    assign t56 = {t82, t81, t80, t79};
    assign t83 = t4[0];
    assign t84 = t1[0];
    assign t85 = t4[1];
    assign t86 = t1[1];
    assign t87 = t4[2];
    assign t88 = t1[2];
    assign t89 = t4[3];
    assign t90 = t1[3];
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) __t91 (
        .I0(t83),
        .I1(t84),
        .I2(t88),
        .I3(t87),
        .I4(t86),
        .I5(t85),
        .O(t91)
    );
    LUT3 # (
        .INIT(8'h90)
    ) __t78 (
        .I0(t89),
        .I1(t90),
        .I2(t91),
        .O(t78)
    );
    assign y = t4;
endmodule
