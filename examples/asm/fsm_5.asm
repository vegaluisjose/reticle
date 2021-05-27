def main(i0:bool, i1:bool, i2:bool, i3:bool, i4:bool) -> (y:i4) {
    t21:i4 = lmuxrega_i4(t15, t0, t19, t5) @lut(??, ??);
    t5:bool = const[1];
    t15:bool = land_bool(t10, i4) @lut(??, ??);
    t0:i4 = const[0];
    t19:i4 = lmux_i4(t14, t4, t18) @lut(??, ??);
    t10:bool = leq_i4(t21, t4) @lut(??, ??);
    t14:bool = land_bool(t9, i3) @lut(??, ??);
    t4:i4 = const[4];
    t18:i4 = lmux_i4(t13, t3, t17) @lut(??, ??);
    t9:bool = leq_i4(t21, t3) @lut(??, ??);
    t13:bool = land_bool(t8, i2) @lut(??, ??);
    t3:i4 = const[3];
    t17:i4 = lmux_i4(t12, t2, t16) @lut(??, ??);
    t8:bool = leq_i4(t21, t2) @lut(??, ??);
    t12:bool = land_bool(t7, i1) @lut(??, ??);
    t2:i4 = const[2];
    t16:i4 = lmux_i4(t11, t1, t21) @lut(??, ??);
    t7:bool = leq_i4(t21, t1) @lut(??, ??);
    t11:bool = land_bool(t6, i0) @lut(??, ??);
    t1:i4 = const[1];
    t6:bool = leq_i4(t21, t0) @lut(??, ??);
    y:i4 = id(t21);
}