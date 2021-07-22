pat lrom_i8i3(a:i3) -> (y:i8) {
    y:i8 = rom(a) @lram;
}

pat brom_i8i8(a:i8) -> (y:i8) {
    y:i8 = rom(a) @bram;
}
