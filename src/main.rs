use reticle::lang::ast::*;
use reticle::inputs;
use std::str::FromStr;
//use reticle::passes::selection;

fn main() {
    let x = inputs!(
        "a" => "i8",
        "b" => "i8"
    );
    println!("{:?}", x);
    //selection::main();
}
