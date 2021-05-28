module main (
    input wire clock,
    input wire reset,
    input wire i0,
    input wire i1,
    input wire i2,
    input wire i3,
    input wire i4,
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
    wire t69;
    wire [3:0] t28;
    wire t70;
    wire t71;
    wire t72;
    wire t73;
    wire t74;
    wire t75;
    wire t76;
    wire t77;
    wire t78;
    wire t50;
    wire t55;
    wire t80;
    wire t81;
    wire t82;
    wire t83;
    wire [3:0] t56;
    wire t87;
    wire t88;
    wire t89;
    wire t90;
    wire t91;
    wire t92;
    wire t93;
    wire t94;
    wire t95;
    wire t96;
    wire t97;
    wire t98;
    wire [3:0] t57;
    wire t99;
    wire t100;
    wire t101;
    wire t102;
    wire t103;
    wire t104;
    wire t105;
    wire t106;
    wire t107;
    wire t79;
    wire t84;
    wire t109;
    wire t110;
    wire t111;
    wire t112;
    wire [3:0] t85;
    wire t115;
    wire t116;
    wire t117;
    wire t118;
    wire t119;
    wire t120;
    wire t121;
    wire t122;
    wire t123;
    wire t124;
    wire t125;
    wire t126;
    wire [3:0] t86;
    wire t127;
    wire t128;
    wire t129;
    wire t130;
    wire t131;
    wire t132;
    wire t133;
    wire t134;
    wire t135;
    wire t108;
    wire t113;
    wire t137;
    wire t138;
    wire t139;
    wire t140;
    wire [3:0] t114;
    wire t141;
    wire t142;
    wire t143;
    wire t144;
    wire t145;
    wire t146;
    wire t147;
    wire t148;
    wire t149;
    wire t136;
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
        .I1(i4),
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
        .I1(i3),
        .O(t26)
    );
    assign t51 = gnd;
    assign t52 = gnd;
    assign t53 = vcc;
    assign t54 = gnd;
    assign t27 = {t54, t53, t52, t51};
    assign t58 = t56[0];
    assign t59 = t57[0];
    assign t60 = t56[1];
    assign t61 = t57[1];
    assign t62 = t56[2];
    assign t63 = t57[2];
    assign t64 = t56[3];
    assign t65 = t57[3];
    LUT3 # (
        .INIT(8'hAC)
    ) __t66 (
        .I0(t58),
        .I1(t59),
        .I2(t55),
        .O(t66)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t67 (
        .I0(t60),
        .I1(t61),
        .I2(t55),
        .O(t67)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t68 (
        .I0(t62),
        .I1(t63),
        .I2(t55),
        .O(t68)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t69 (
        .I0(t64),
        .I1(t65),
        .I2(t55),
        .O(t69)
    );
    assign t28 = {t69, t68, t67, t66};
    assign t70 = t4[0];
    assign t71 = t56[0];
    assign t72 = t4[1];
    assign t73 = t56[1];
    assign t74 = t4[2];
    assign t75 = t56[2];
    assign t76 = t4[3];
    assign t77 = t56[3];
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) __t78 (
        .I0(t70),
        .I1(t71),
        .I2(t75),
        .I3(t74),
        .I4(t73),
        .I5(t72),
        .O(t78)
    );
    LUT3 # (
        .INIT(8'h90)
    ) __t50 (
        .I0(t76),
        .I1(t77),
        .I2(t78),
        .O(t50)
    );
    LUT2 # (
        .INIT(4'h8)
    ) __t55 (
        .I0(t79),
        .I1(i2),
        .O(t55)
    );
    assign t80 = vcc;
    assign t81 = vcc;
    assign t82 = gnd;
    assign t83 = gnd;
    assign t56 = {t83, t82, t81, t80};
    assign t87 = t85[0];
    assign t88 = t86[0];
    assign t89 = t85[1];
    assign t90 = t86[1];
    assign t91 = t85[2];
    assign t92 = t86[2];
    assign t93 = t85[3];
    assign t94 = t86[3];
    LUT3 # (
        .INIT(8'hAC)
    ) __t95 (
        .I0(t87),
        .I1(t88),
        .I2(t84),
        .O(t95)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t96 (
        .I0(t89),
        .I1(t90),
        .I2(t84),
        .O(t96)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t97 (
        .I0(t91),
        .I1(t92),
        .I2(t84),
        .O(t97)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t98 (
        .I0(t93),
        .I1(t94),
        .I2(t84),
        .O(t98)
    );
    assign t57 = {t98, t97, t96, t95};
    assign t99 = t4[0];
    assign t100 = t85[0];
    assign t101 = t4[1];
    assign t102 = t85[1];
    assign t103 = t4[2];
    assign t104 = t85[2];
    assign t105 = t4[3];
    assign t106 = t85[3];
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) __t107 (
        .I0(t99),
        .I1(t100),
        .I2(t104),
        .I3(t103),
        .I4(t102),
        .I5(t101),
        .O(t107)
    );
    LUT3 # (
        .INIT(8'h90)
    ) __t79 (
        .I0(t105),
        .I1(t106),
        .I2(t107),
        .O(t79)
    );
    LUT2 # (
        .INIT(4'h8)
    ) __t84 (
        .I0(t108),
        .I1(i1),
        .O(t84)
    );
    assign t109 = gnd;
    assign t110 = vcc;
    assign t111 = gnd;
    assign t112 = gnd;
    assign t85 = {t112, t111, t110, t109};
    assign t115 = t114[0];
    assign t116 = t4[0];
    assign t117 = t114[1];
    assign t118 = t4[1];
    assign t119 = t114[2];
    assign t120 = t4[2];
    assign t121 = t114[3];
    assign t122 = t4[3];
    LUT3 # (
        .INIT(8'hAC)
    ) __t123 (
        .I0(t115),
        .I1(t116),
        .I2(t113),
        .O(t123)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t124 (
        .I0(t117),
        .I1(t118),
        .I2(t113),
        .O(t124)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t125 (
        .I0(t119),
        .I1(t120),
        .I2(t113),
        .O(t125)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t126 (
        .I0(t121),
        .I1(t122),
        .I2(t113),
        .O(t126)
    );
    assign t86 = {t126, t125, t124, t123};
    assign t127 = t4[0];
    assign t128 = t114[0];
    assign t129 = t4[1];
    assign t130 = t114[1];
    assign t131 = t4[2];
    assign t132 = t114[2];
    assign t133 = t4[3];
    assign t134 = t114[3];
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) __t135 (
        .I0(t127),
        .I1(t128),
        .I2(t132),
        .I3(t131),
        .I4(t130),
        .I5(t129),
        .O(t135)
    );
    LUT3 # (
        .INIT(8'h90)
    ) __t108 (
        .I0(t133),
        .I1(t134),
        .I2(t135),
        .O(t108)
    );
    LUT2 # (
        .INIT(4'h8)
    ) __t113 (
        .I0(t136),
        .I1(i0),
        .O(t113)
    );
    assign t137 = vcc;
    assign t138 = gnd;
    assign t139 = gnd;
    assign t140 = gnd;
    assign t114 = {t140, t139, t138, t137};
    assign t141 = t4[0];
    assign t142 = t1[0];
    assign t143 = t4[1];
    assign t144 = t1[1];
    assign t145 = t4[2];
    assign t146 = t1[2];
    assign t147 = t4[3];
    assign t148 = t1[3];
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) __t149 (
        .I0(t141),
        .I1(t142),
        .I2(t146),
        .I3(t145),
        .I4(t144),
        .I5(t143),
        .O(t149)
    );
    LUT3 # (
        .INIT(8'h90)
    ) __t136 (
        .I0(t147),
        .I1(t148),
        .I2(t149),
        .O(t136)
    );
    assign y = t4;
endmodule
