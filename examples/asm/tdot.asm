def main(a0:i8, c0:i8, e0:i8, g0:i8, i0:i8, a1:i8, c1:i8, e1:i8, g1:i8, i1:i8, a2:i8, c2:i8, e2:i8, g2:i8, i2:i8, b0:i8, d0:i8, f0:i8, h0:i8, j0:i8, b1:i8, d1:i8, f1:i8, h1:i8, j1:i8, b2:i8, d2:i8, f2:i8, h2:i8, j2:i8, m:i8, n:i8, o:i8, p:i8, q:i8, en:bool) -> (v:i8, w:i8, x:i8, y:i8, z:i8) {
    v:i8 = dmuladdrega_i8i8(a2, b2, t11, en) @dsp(??, ??);
    t11:i8 = dmuladdrega_i8i8(a1, b1, t5, en) @dsp(??, ??);
    t5:i8 = dmuladdrega_i8i8(a0, b0, m, en) @dsp(??, ??);
    w:i8 = dmuladdrega_i8i8(c2, d2, t29, en) @dsp(??, ??);
    t29:i8 = dmuladdrega_i8i8(c1, d1, t23, en) @dsp(??, ??);
    t23:i8 = dmuladdrega_i8i8(c0, d0, n, en) @dsp(??, ??);
    x:i8 = dmuladdrega_i8i8(e2, f2, t47, en) @dsp(??, ??);
    t47:i8 = dmuladdrega_i8i8(e1, f1, t41, en) @dsp(??, ??);
    t41:i8 = dmuladdrega_i8i8(e0, f0, o, en) @dsp(??, ??);
    y:i8 = dmuladdrega_i8i8(g2, h2, t65, en) @dsp(??, ??);
    t65:i8 = dmuladdrega_i8i8(g1, h1, t59, en) @dsp(??, ??);
    t59:i8 = dmuladdrega_i8i8(g0, h0, p, en) @dsp(??, ??);
    z:i8 = dmuladdrega_i8i8(i2, j2, t83, en) @dsp(??, ??);
    t83:i8 = dmuladdrega_i8i8(i1, j1, t77, en) @dsp(??, ??);
    t77:i8 = dmuladdrega_i8i8(i0, j0, q, en) @dsp(??, ??);
}
