pat ladd_i8(a:i8, b:i8) -> (y:i8) {
    y:i8 = add(a, b) @lut;
}

pat ladd_i4(a:i4, b:i4) -> (y:i4) {
    y:i4 = add(a, b) @lut;
}

pat laddrega_i8(a:i8, b:i8, en:bool) -> (y:i8) {
    t0:i8 = add(a, b) @lut;
    y:i8 = reg[0](t0, en) @lut;
}

pat laddrega_i4(a:i4, b:i4, en:bool) -> (y:i4) {
    t0:i4 = add(a, b) @lut;
    y:i4 = reg[0](t0, en) @lut;
}

pat lrega_i8(a:i8, en:bool) -> (y:i8) {
    y:i8 = reg[0](a, en) @lut;
}

pat lrega_i4(a:i4, en:bool) -> (y:i4) {
    y:i4 = reg[0](a, en) @lut;
}

pat lrega_i128(a:i128, en:bool) -> (y:i128) {
    y:i128 = reg[0](a, en) @lut;
}

pat leq_i2(a:i2, b:i2) -> (y:bool) {
    y:bool = eq(a, b) @lut;
}

pat leq_i4(a:i4, b:i4) -> (y:bool) {
    y:bool = eq(a, b) @lut;
}

pat leq_bool(a:bool, b:bool) -> (y:bool) {
    y:bool = eq(a, b) @lut;
}

pat land_bool(a:bool, b:bool) -> (y:bool) {
    y:bool = and(a, b) @lut;
}

pat lmux_i2(sel:bool, a:i2, b:i2) -> (y:i2) {
    y:i2 = mux(sel, a, b) @lut;
}

pat lmux_i4(sel:bool, a:i4, b:i4) -> (y:i4) {
    y:i4 = mux(sel, a, b) @lut;
}

pat lmux_i8(sel:bool, a:i8, b:i8) -> (y:i8) {
    y:i8 = mux(sel, a, b) @lut;
}

pat lmux_i128(sel:bool, a:i128, b:i128) -> (y:i128) {
    y:i128 = mux(sel, a, b) @lut;
}

pat lmuxrega_i2(sel:bool, a:i2, b:i2, en:bool) -> (y:i2) {
    t0:i2 = mux(sel, a, b) @lut;
    y:i2 = reg[0](t0, en) @lut;
}

pat lmuxrega_i4(sel:bool, a:i4, b:i4, en:bool) -> (y:i4) {
    t0:i4 = mux(sel, a, b) @lut;
    y:i4 = reg[0](t0, en) @lut;
}

pat lmuxrega_i8(sel:bool, a:i8, b:i8, en:bool) -> (y:i8) {
    t0:i8 = mux(sel, a, b) @lut;
    y:i8 = reg[0](t0, en) @lut;
}

pat lmuxrega_i128(sel:bool, a:i128, b:i128, en:bool) -> (y:i128) {
    t0:i128 = mux(sel, a, b) @lut;
    y:i128 = reg[0](t0, en) @lut;
}
