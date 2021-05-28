module main (
    input wire clock,
    input wire reset,
    input wire i0,
    input wire i1,
    input wire i2,
    input wire i3,
    input wire i4,
    input wire i5,
    input wire i6,
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
    wire t127;
    wire [3:0] t86;
    wire t128;
    wire t129;
    wire t130;
    wire t131;
    wire t132;
    wire t133;
    wire t134;
    wire t135;
    wire t136;
    wire t108;
    wire t113;
    wire t138;
    wire t139;
    wire t140;
    wire t141;
    wire [3:0] t114;
    wire t145;
    wire t146;
    wire t147;
    wire t148;
    wire t149;
    wire t150;
    wire t151;
    wire t152;
    wire t153;
    wire t154;
    wire t155;
    wire t156;
    wire [3:0] t115;
    wire t157;
    wire t158;
    wire t159;
    wire t160;
    wire t161;
    wire t162;
    wire t163;
    wire t164;
    wire t165;
    wire t137;
    wire t142;
    wire t167;
    wire t168;
    wire t169;
    wire t170;
    wire [3:0] t143;
    wire t173;
    wire t174;
    wire t175;
    wire t176;
    wire t177;
    wire t178;
    wire t179;
    wire t180;
    wire t181;
    wire t182;
    wire t183;
    wire t184;
    wire [3:0] t144;
    wire t185;
    wire t186;
    wire t187;
    wire t188;
    wire t189;
    wire t190;
    wire t191;
    wire t192;
    wire t193;
    wire t166;
    wire t171;
    wire t195;
    wire t196;
    wire t197;
    wire t198;
    wire [3:0] t172;
    wire t199;
    wire t200;
    wire t201;
    wire t202;
    wire t203;
    wire t204;
    wire t205;
    wire t206;
    wire t207;
    wire t194;
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
        .I1(i6),
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
        .I1(i5),
        .O(t26)
    );
    assign t51 = gnd;
    assign t52 = vcc;
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
        .I1(i4),
        .O(t55)
    );
    assign t80 = vcc;
    assign t81 = gnd;
    assign t82 = vcc;
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
        .I1(i3),
        .O(t84)
    );
    assign t109 = gnd;
    assign t110 = gnd;
    assign t111 = vcc;
    assign t112 = gnd;
    assign t85 = {t112, t111, t110, t109};
    assign t116 = t114[0];
    assign t117 = t115[0];
    assign t118 = t114[1];
    assign t119 = t115[1];
    assign t120 = t114[2];
    assign t121 = t115[2];
    assign t122 = t114[3];
    assign t123 = t115[3];
    LUT3 # (
        .INIT(8'hAC)
    ) __t124 (
        .I0(t116),
        .I1(t117),
        .I2(t113),
        .O(t124)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t125 (
        .I0(t118),
        .I1(t119),
        .I2(t113),
        .O(t125)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t126 (
        .I0(t120),
        .I1(t121),
        .I2(t113),
        .O(t126)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t127 (
        .I0(t122),
        .I1(t123),
        .I2(t113),
        .O(t127)
    );
    assign t86 = {t127, t126, t125, t124};
    assign t128 = t4[0];
    assign t129 = t114[0];
    assign t130 = t4[1];
    assign t131 = t114[1];
    assign t132 = t4[2];
    assign t133 = t114[2];
    assign t134 = t4[3];
    assign t135 = t114[3];
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) __t136 (
        .I0(t128),
        .I1(t129),
        .I2(t133),
        .I3(t132),
        .I4(t131),
        .I5(t130),
        .O(t136)
    );
    LUT3 # (
        .INIT(8'h90)
    ) __t108 (
        .I0(t134),
        .I1(t135),
        .I2(t136),
        .O(t108)
    );
    LUT2 # (
        .INIT(4'h8)
    ) __t113 (
        .I0(t137),
        .I1(i2),
        .O(t113)
    );
    assign t138 = vcc;
    assign t139 = vcc;
    assign t140 = gnd;
    assign t141 = gnd;
    assign t114 = {t141, t140, t139, t138};
    assign t145 = t143[0];
    assign t146 = t144[0];
    assign t147 = t143[1];
    assign t148 = t144[1];
    assign t149 = t143[2];
    assign t150 = t144[2];
    assign t151 = t143[3];
    assign t152 = t144[3];
    LUT3 # (
        .INIT(8'hAC)
    ) __t153 (
        .I0(t145),
        .I1(t146),
        .I2(t142),
        .O(t153)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t154 (
        .I0(t147),
        .I1(t148),
        .I2(t142),
        .O(t154)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t155 (
        .I0(t149),
        .I1(t150),
        .I2(t142),
        .O(t155)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t156 (
        .I0(t151),
        .I1(t152),
        .I2(t142),
        .O(t156)
    );
    assign t115 = {t156, t155, t154, t153};
    assign t157 = t4[0];
    assign t158 = t143[0];
    assign t159 = t4[1];
    assign t160 = t143[1];
    assign t161 = t4[2];
    assign t162 = t143[2];
    assign t163 = t4[3];
    assign t164 = t143[3];
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) __t165 (
        .I0(t157),
        .I1(t158),
        .I2(t162),
        .I3(t161),
        .I4(t160),
        .I5(t159),
        .O(t165)
    );
    LUT3 # (
        .INIT(8'h90)
    ) __t137 (
        .I0(t163),
        .I1(t164),
        .I2(t165),
        .O(t137)
    );
    LUT2 # (
        .INIT(4'h8)
    ) __t142 (
        .I0(t166),
        .I1(i1),
        .O(t142)
    );
    assign t167 = gnd;
    assign t168 = vcc;
    assign t169 = gnd;
    assign t170 = gnd;
    assign t143 = {t170, t169, t168, t167};
    assign t173 = t172[0];
    assign t174 = t4[0];
    assign t175 = t172[1];
    assign t176 = t4[1];
    assign t177 = t172[2];
    assign t178 = t4[2];
    assign t179 = t172[3];
    assign t180 = t4[3];
    LUT3 # (
        .INIT(8'hAC)
    ) __t181 (
        .I0(t173),
        .I1(t174),
        .I2(t171),
        .O(t181)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t182 (
        .I0(t175),
        .I1(t176),
        .I2(t171),
        .O(t182)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t183 (
        .I0(t177),
        .I1(t178),
        .I2(t171),
        .O(t183)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t184 (
        .I0(t179),
        .I1(t180),
        .I2(t171),
        .O(t184)
    );
    assign t144 = {t184, t183, t182, t181};
    assign t185 = t4[0];
    assign t186 = t172[0];
    assign t187 = t4[1];
    assign t188 = t172[1];
    assign t189 = t4[2];
    assign t190 = t172[2];
    assign t191 = t4[3];
    assign t192 = t172[3];
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) __t193 (
        .I0(t185),
        .I1(t186),
        .I2(t190),
        .I3(t189),
        .I4(t188),
        .I5(t187),
        .O(t193)
    );
    LUT3 # (
        .INIT(8'h90)
    ) __t166 (
        .I0(t191),
        .I1(t192),
        .I2(t193),
        .O(t166)
    );
    LUT2 # (
        .INIT(4'h8)
    ) __t171 (
        .I0(t194),
        .I1(i0),
        .O(t171)
    );
    assign t195 = vcc;
    assign t196 = gnd;
    assign t197 = gnd;
    assign t198 = gnd;
    assign t172 = {t198, t197, t196, t195};
    assign t199 = t4[0];
    assign t200 = t1[0];
    assign t201 = t4[1];
    assign t202 = t1[1];
    assign t203 = t4[2];
    assign t204 = t1[2];
    assign t205 = t4[3];
    assign t206 = t1[3];
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) __t207 (
        .I0(t199),
        .I1(t200),
        .I2(t204),
        .I3(t203),
        .I4(t202),
        .I5(t201),
        .O(t207)
    );
    LUT3 # (
        .INIT(8'h90)
    ) __t194 (
        .I0(t205),
        .I1(t206),
        .I2(t207),
        .O(t194)
    );
    assign y = t4;
endmodule
