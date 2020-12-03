use reticle::v2::il::ast::Ty;
use std::str::FromStr;

fn main() {
    let x = Ty::from_str("i8");
    println!("{:?}", x);
}
