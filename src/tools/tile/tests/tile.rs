use asm::errors::Error;
use asm::parser::Parser;
use tile::tile_from_prog;

fn test(input: &str, output: &str) -> Result<(), Error> {
    let prog = Parser::parse_from_str(input)?;
    let res = tile_from_prog(&prog);
    let exp = Parser::parse_from_str(output)?;
    assert_eq!(res, exp);
    Ok(())
}

#[test]
fn tile_xor128() -> Result<(), Error> {
    let input = r#"def main(a:i128, b:i128) -> (y:i128) {
    y:i128 = lxor_i128(a, b) @lut(??, ??);
}"#;
    let output = r#"def main(a:i128, b:i128) -> (y:i128) {
    t0:i8 = ext[0, 7](a);
    t1:i8 = ext[0, 7](b);
    t2:i8 = lxor_i8(t0, t1) @lut(??, ??);
    t3:i8 = ext[8, 15](a);
    t4:i8 = ext[8, 15](b);
    t5:i8 = lxor_i8(t3, t4) @lut(??, ??);
    t6:i8 = ext[16, 23](a);
    t7:i8 = ext[16, 23](b);
    t8:i8 = lxor_i8(t6, t7) @lut(??, ??);
    t9:i8 = ext[24, 31](a);
    t10:i8 = ext[24, 31](b);
    t11:i8 = lxor_i8(t9, t10) @lut(??, ??);
    t12:i8 = ext[32, 39](a);
    t13:i8 = ext[32, 39](b);
    t14:i8 = lxor_i8(t12, t13) @lut(??, ??);
    t15:i8 = ext[40, 47](a);
    t16:i8 = ext[40, 47](b);
    t17:i8 = lxor_i8(t15, t16) @lut(??, ??);
    t18:i8 = ext[48, 55](a);
    t19:i8 = ext[48, 55](b);
    t20:i8 = lxor_i8(t18, t19) @lut(??, ??);
    t21:i8 = ext[56, 63](a);
    t22:i8 = ext[56, 63](b);
    t23:i8 = lxor_i8(t21, t22) @lut(??, ??);
    t24:i8 = ext[64, 71](a);
    t25:i8 = ext[64, 71](b);
    t26:i8 = lxor_i8(t24, t25) @lut(??, ??);
    t27:i8 = ext[72, 79](a);
    t28:i8 = ext[72, 79](b);
    t29:i8 = lxor_i8(t27, t28) @lut(??, ??);
    t30:i8 = ext[80, 87](a);
    t31:i8 = ext[80, 87](b);
    t32:i8 = lxor_i8(t30, t31) @lut(??, ??);
    t33:i8 = ext[88, 95](a);
    t34:i8 = ext[88, 95](b);
    t35:i8 = lxor_i8(t33, t34) @lut(??, ??);
    t36:i8 = ext[96, 103](a);
    t37:i8 = ext[96, 103](b);
    t38:i8 = lxor_i8(t36, t37) @lut(??, ??);
    t39:i8 = ext[104, 111](a);
    t40:i8 = ext[104, 111](b);
    t41:i8 = lxor_i8(t39, t40) @lut(??, ??);
    t42:i8 = ext[112, 119](a);
    t43:i8 = ext[112, 119](b);
    t44:i8 = lxor_i8(t42, t43) @lut(??, ??);
    t45:i8 = ext[120, 127](a);
    t46:i8 = ext[120, 127](b);
    t47:i8 = lxor_i8(t45, t46) @lut(??, ??);
    y:i128 = cat(t2, t5, t8, t11, t14, t17, t20, t23, t26, t29, t32, t35, t38, t41, t44, t47);
}"#;
    Ok(test(input, output)?)
}

#[test]
fn tile_xor32() -> Result<(), Error> {
    let input = r#"def main(a:i32, b:i32) -> (y:i32) {
    y:i32 = lxor_i32(a, b) @lut(??, ??);
}"#;
    let output = r#"def main(a:i32, b:i32) -> (y:i32) {
    t0:i8 = ext[0, 7](a);
    t1:i8 = ext[0, 7](b);
    t2:i8 = lxor_i8(t0, t1) @lut(??, ??);
    t3:i8 = ext[8, 15](a);
    t4:i8 = ext[8, 15](b);
    t5:i8 = lxor_i8(t3, t4) @lut(??, ??);
    t6:i8 = ext[16, 23](a);
    t7:i8 = ext[16, 23](b);
    t8:i8 = lxor_i8(t6, t7) @lut(??, ??);
    t9:i8 = ext[24, 31](a);
    t10:i8 = ext[24, 31](b);
    t11:i8 = lxor_i8(t9, t10) @lut(??, ??);
    y:i32 = cat(t2, t5, t8, t11);
}"#;
    Ok(test(input, output)?)
}
