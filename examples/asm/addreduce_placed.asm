def main(a:i8, b:i8, c:i8, d:i8, e:i8, f:i8, g:i8, h:i8, en:bool) -> (y:i8) {
    y:i8 = laddrega_i8(t10, t11, en) @lut(0, 0);
    t10:i8 = laddrega_i8(t4, t5, en) @lut(0, 1);
    t11:i8 = laddrega_i8(t6, t7, en) @lut(0, 2);
    t4:i8 = laddrega_i8(a, b, en) @lut(0, 3);
    t5:i8 = laddrega_i8(c, d, en) @lut(0, 4);
    t6:i8 = laddrega_i8(e, f, en) @lut(0, 5);
    t7:i8 = laddrega_i8(g, h, en) @lut(0, 6);
}
