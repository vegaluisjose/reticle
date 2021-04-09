pat ladd_i8(a: i8, b: i8) -> (y: i8) {
    y:i8 = add(a, b) @lut;
}

pat ladd_i8_reg(a: i8, b: i8, en: bool) -> (y: i8) {
    t0:i8 = add(a, b) @lut;
    y:i8 = reg[0](t0, en) @lut;
}

