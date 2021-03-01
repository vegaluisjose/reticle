def main(a0:i8, b0:i8, a1:i8, b1:i8, c0:i8, en:bool) -> (y:i8) {
    t5:i8 = dsp_muladd_i8_ra_rb_rm_rp(a0, b0, c0, en) @dsp(??, ??);
    y:i8 = dsp_muladd_i8_ra_rb_rm_rp(a1, b1, t5, en) @dsp(??, ??);
}