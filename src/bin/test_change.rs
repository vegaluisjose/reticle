use ir::parser::Parser as IrParser;
use asm::parser::Parser as AsmParser;
use isel::select;

fn main() {
    let prog = IrParser::parse_from_file("examples/ir/lut_add.ir").unwrap();
    let exp = AsmParser::parse_from_file("examples/asm/lut_add.asm").unwrap();
    let res = select(&prog).unwrap();
    println!("{}", exp);
    println!("{}", res);
}
