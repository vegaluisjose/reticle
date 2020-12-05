use reticle::v2::il::parser::parse;

fn main() {
    let prog = parse("main() -> (y:i8) { y:i8 = const[-3]; }");
    println!("{}", prog);
}
