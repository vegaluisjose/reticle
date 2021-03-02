def main(i0:bool, i1:bool, i2:bool) -> (y:i2) {
    t0:i2 = const[0];
    t4:bool = lut_eq_i2(t13, t0) @lut(??, ??);
    t1:i2 = const[1];
    t7:bool = lut_and_b(t4, i0) @lut(??, ??);
    t5:bool = lut_eq_i2(t13, t1) @lut(??, ??);
    t2:i2 = const[2];
    t10:i2 = lut_mux_i2(t7, t1, t13) @lut(??, ??);
    t8:bool = lut_and_b(t5, i1) @lut(??, ??);
    t6:bool = lut_eq_i2(t13, t2) @lut(??, ??);
    t11:i2 = lut_mux_i2(t8, t2, t10) @lut(??, ??);
    t9:bool = lut_and_b(t6, i2) @lut(??, ??);
    t3:bool = const[1];
    t13:i2 = lut_mux_reg_i2(t9, t0, t11, t3) @lut(??, ??);
    y:i2 = id(t13);
}