pat dmul_i8(a: i8, b: i8) -> (y: i8) {
    y:i8 = mul(a, b) @dsp;
}

pat dmuladd_i8i8(a: i8, b: i8, c: i8) -> (y: i8) {
    t0:i8 = mul(a, b) @dsp;
    y:i8 = add(t0, c) @dsp;
}

