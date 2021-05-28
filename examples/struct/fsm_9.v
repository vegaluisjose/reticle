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
    input wire i7,
    input wire i8,
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
    wire t185;
    wire [3:0] t144;
    wire t186;
    wire t187;
    wire t188;
    wire t189;
    wire t190;
    wire t191;
    wire t192;
    wire t193;
    wire t194;
    wire t166;
    wire t171;
    wire t196;
    wire t197;
    wire t198;
    wire t199;
    wire [3:0] t172;
    wire t203;
    wire t204;
    wire t205;
    wire t206;
    wire t207;
    wire t208;
    wire t209;
    wire t210;
    wire t211;
    wire t212;
    wire t213;
    wire t214;
    wire [3:0] t173;
    wire t215;
    wire t216;
    wire t217;
    wire t218;
    wire t219;
    wire t220;
    wire t221;
    wire t222;
    wire t223;
    wire t195;
    wire t200;
    wire t225;
    wire t226;
    wire t227;
    wire t228;
    wire [3:0] t201;
    wire t231;
    wire t232;
    wire t233;
    wire t234;
    wire t235;
    wire t236;
    wire t237;
    wire t238;
    wire t239;
    wire t240;
    wire t241;
    wire t242;
    wire [3:0] t202;
    wire t243;
    wire t244;
    wire t245;
    wire t246;
    wire t247;
    wire t248;
    wire t249;
    wire t250;
    wire t251;
    wire t224;
    wire t229;
    wire t253;
    wire t254;
    wire t255;
    wire t256;
    wire [3:0] t230;
    wire t257;
    wire t258;
    wire t259;
    wire t260;
    wire t261;
    wire t262;
    wire t263;
    wire t264;
    wire t265;
    wire t252;
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
        .I1(i8),
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
        .I1(i7),
        .O(t26)
    );
    assign t51 = gnd;
    assign t52 = gnd;
    assign t53 = gnd;
    assign t54 = vcc;
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
        .I1(i6),
        .O(t55)
    );
    assign t80 = vcc;
    assign t81 = vcc;
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
        .I1(i5),
        .O(t84)
    );
    assign t109 = gnd;
    assign t110 = vcc;
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
        .I1(i4),
        .O(t113)
    );
    assign t138 = vcc;
    assign t139 = gnd;
    assign t140 = vcc;
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
        .I1(i3),
        .O(t142)
    );
    assign t167 = gnd;
    assign t168 = gnd;
    assign t169 = vcc;
    assign t170 = gnd;
    assign t143 = {t170, t169, t168, t167};
    assign t174 = t172[0];
    assign t175 = t173[0];
    assign t176 = t172[1];
    assign t177 = t173[1];
    assign t178 = t172[2];
    assign t179 = t173[2];
    assign t180 = t172[3];
    assign t181 = t173[3];
    LUT3 # (
        .INIT(8'hAC)
    ) __t182 (
        .I0(t174),
        .I1(t175),
        .I2(t171),
        .O(t182)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t183 (
        .I0(t176),
        .I1(t177),
        .I2(t171),
        .O(t183)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t184 (
        .I0(t178),
        .I1(t179),
        .I2(t171),
        .O(t184)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t185 (
        .I0(t180),
        .I1(t181),
        .I2(t171),
        .O(t185)
    );
    assign t144 = {t185, t184, t183, t182};
    assign t186 = t4[0];
    assign t187 = t172[0];
    assign t188 = t4[1];
    assign t189 = t172[1];
    assign t190 = t4[2];
    assign t191 = t172[2];
    assign t192 = t4[3];
    assign t193 = t172[3];
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) __t194 (
        .I0(t186),
        .I1(t187),
        .I2(t191),
        .I3(t190),
        .I4(t189),
        .I5(t188),
        .O(t194)
    );
    LUT3 # (
        .INIT(8'h90)
    ) __t166 (
        .I0(t192),
        .I1(t193),
        .I2(t194),
        .O(t166)
    );
    LUT2 # (
        .INIT(4'h8)
    ) __t171 (
        .I0(t195),
        .I1(i2),
        .O(t171)
    );
    assign t196 = vcc;
    assign t197 = vcc;
    assign t198 = gnd;
    assign t199 = gnd;
    assign t172 = {t199, t198, t197, t196};
    assign t203 = t201[0];
    assign t204 = t202[0];
    assign t205 = t201[1];
    assign t206 = t202[1];
    assign t207 = t201[2];
    assign t208 = t202[2];
    assign t209 = t201[3];
    assign t210 = t202[3];
    LUT3 # (
        .INIT(8'hAC)
    ) __t211 (
        .I0(t203),
        .I1(t204),
        .I2(t200),
        .O(t211)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t212 (
        .I0(t205),
        .I1(t206),
        .I2(t200),
        .O(t212)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t213 (
        .I0(t207),
        .I1(t208),
        .I2(t200),
        .O(t213)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t214 (
        .I0(t209),
        .I1(t210),
        .I2(t200),
        .O(t214)
    );
    assign t173 = {t214, t213, t212, t211};
    assign t215 = t4[0];
    assign t216 = t201[0];
    assign t217 = t4[1];
    assign t218 = t201[1];
    assign t219 = t4[2];
    assign t220 = t201[2];
    assign t221 = t4[3];
    assign t222 = t201[3];
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) __t223 (
        .I0(t215),
        .I1(t216),
        .I2(t220),
        .I3(t219),
        .I4(t218),
        .I5(t217),
        .O(t223)
    );
    LUT3 # (
        .INIT(8'h90)
    ) __t195 (
        .I0(t221),
        .I1(t222),
        .I2(t223),
        .O(t195)
    );
    LUT2 # (
        .INIT(4'h8)
    ) __t200 (
        .I0(t224),
        .I1(i1),
        .O(t200)
    );
    assign t225 = gnd;
    assign t226 = vcc;
    assign t227 = gnd;
    assign t228 = gnd;
    assign t201 = {t228, t227, t226, t225};
    assign t231 = t230[0];
    assign t232 = t4[0];
    assign t233 = t230[1];
    assign t234 = t4[1];
    assign t235 = t230[2];
    assign t236 = t4[2];
    assign t237 = t230[3];
    assign t238 = t4[3];
    LUT3 # (
        .INIT(8'hAC)
    ) __t239 (
        .I0(t231),
        .I1(t232),
        .I2(t229),
        .O(t239)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t240 (
        .I0(t233),
        .I1(t234),
        .I2(t229),
        .O(t240)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t241 (
        .I0(t235),
        .I1(t236),
        .I2(t229),
        .O(t241)
    );
    LUT3 # (
        .INIT(8'hAC)
    ) __t242 (
        .I0(t237),
        .I1(t238),
        .I2(t229),
        .O(t242)
    );
    assign t202 = {t242, t241, t240, t239};
    assign t243 = t4[0];
    assign t244 = t230[0];
    assign t245 = t4[1];
    assign t246 = t230[1];
    assign t247 = t4[2];
    assign t248 = t230[2];
    assign t249 = t4[3];
    assign t250 = t230[3];
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) __t251 (
        .I0(t243),
        .I1(t244),
        .I2(t248),
        .I3(t247),
        .I4(t246),
        .I5(t245),
        .O(t251)
    );
    LUT3 # (
        .INIT(8'h90)
    ) __t224 (
        .I0(t249),
        .I1(t250),
        .I2(t251),
        .O(t224)
    );
    LUT2 # (
        .INIT(4'h8)
    ) __t229 (
        .I0(t252),
        .I1(i0),
        .O(t229)
    );
    assign t253 = vcc;
    assign t254 = gnd;
    assign t255 = gnd;
    assign t256 = gnd;
    assign t230 = {t256, t255, t254, t253};
    assign t257 = t4[0];
    assign t258 = t1[0];
    assign t259 = t4[1];
    assign t260 = t1[1];
    assign t261 = t4[2];
    assign t262 = t1[2];
    assign t263 = t4[3];
    assign t264 = t1[3];
    LUT6 # (
        .INIT(64'h9009000000009009)
    ) __t265 (
        .I0(t257),
        .I1(t258),
        .I2(t262),
        .I3(t261),
        .I4(t260),
        .I5(t259),
        .O(t265)
    );
    LUT3 # (
        .INIT(8'h90)
    ) __t252 (
        .I0(t263),
        .I1(t264),
        .I2(t265),
        .O(t252)
    );
    assign y = t4;
endmodule
