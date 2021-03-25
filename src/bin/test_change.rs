use asm::ast::Id;

fn main() {
    let mut x = Id::new();
    x.push_str("hello");
    println!("{}", x);
}
