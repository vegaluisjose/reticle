def main(i0:bool, i1:bool, i2:bool) -> (y:i4) {
    t13:i4 = lmuxrega_i4(t9, t0, t11, t3) @lut(??, ??);
    t3:bool = const[1];
    t9:bool = land_bool(t6, i2) @lut(??, ??);
    t0:i4 = const[0];
    t11:i4 = lmux_i4(t8, t2, t10) @lut(??, ??);
    t6:bool = leq_i4(t13, t2) @lut(??, ??);
    t8:bool = land_bool(t5, i1) @lut(??, ??);
    t2:i4 = const[2];
    t10:i4 = lmux_i4(t7, t1, t13) @lut(??, ??);
    t5:bool = leq_i4(t13, t1) @lut(??, ??);
    t7:bool = land_bool(t4, i0) @lut(??, ??);
    t1:i4 = const[1];
    t4:bool = leq_i4(t13, t0) @lut(??, ??);
    y:i4 = id(t13);
}