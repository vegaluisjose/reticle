pat ladd_i8(a: i8, b: i8) -> (y: i8) {
    y:i8 = add(a, b) @lut;
}

pat laddrega_i8(a: i8, b: i8, en: bool) -> (y: i8) {
    t0:i8 = add(a, b) @lut;
    y:i8 = reg[0](t0, en) @lut;
}

pat lrega_i8(a: i8, en: bool) -> (y: i8) {
    y:i8 = reg[0](a, en) @lut;
}
