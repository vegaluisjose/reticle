pat dmul_i8(a: i8, b: i8) -> (y: i8) {
    y:i8 = mul(a, b) @dsp;
}

pat dmuladd_i8i8(a: i8, b: i8, c: i8) -> (y: i8) {
    t0:i8 = mul(a, b) @dsp;
    y:i8 = add(t0, c) @dsp;
}

pat daddrega_i8v4(a: i8<4>, b: i8<4>, en: bool) -> (y: i8<4>) {
    t0:i8<4> = reg[0](a, en) @dsp;
    t1:i8<4> = reg[0](b, en) @dsp;
    t2:i8<4> = add(t0, t1) @dsp;
    y:i8<4> = reg[0](t2, en) @dsp;
}

pat dmuladdrega_i8i8(a: i8, b: i8, c: i8, en: bool) -> (y: i8) {
    t0:i8 = reg[0](a, en) @dsp;
    t1:i8 = reg[0](b, en) @dsp;
    t2:i8 = mul(t0, t1) @dsp;
    t3:i8 = reg[0](t2, en) @dsp;
    t4:i8 = add(t3, c) @dsp;
    y:i8 = reg[0](t4, en) @dsp;
}
