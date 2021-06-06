def main(en:bool, a:i8<4>, b:i8<4>) -> (y:i8<4>) {
    y:i8<4> = daddrega_i8v4(a, b, en, en, en) @dsp(??, ??);
}