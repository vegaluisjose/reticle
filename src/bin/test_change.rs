use ir::parser::Parser;
use isel::tree::select;

fn main() {
    let prog = Parser::parse_from_file("examples/ir/add.ir").unwrap();
    let asm = select(&prog).unwrap();
    println!("{}", prog);
    println!("\n\n\n");
    println!("{}", asm);
}
