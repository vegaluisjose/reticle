def main(a0:i8, a1:i8, a2:i8, b0:i8, b1:i8, b2:i8, c:i8, en:bool) -> (y:i8) {
    y:i8 = dmuladdrega_i8i8(a2, b2, t11, en, en, en, en) @dsp(??, ??);
    t11:i8 = dmuladdrega_i8i8(a1, b1, t5, en, en, en, en) @dsp(??, ??);
    t5:i8 = dmuladdrega_i8i8(a0, b0, c, en, en, en, en) @dsp(??, ??);
}