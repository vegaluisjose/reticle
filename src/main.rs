//use reticle::lang::ast::macros;
use reticle::lang::ast;
use reticle::passes::selection;
use std::str::FromStr;

fn main() {
    let x = ast::DataType::from_str("u8").unwrap();
    println!("{}", x);
    selection::main();
}
