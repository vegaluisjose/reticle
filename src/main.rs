use reticle::lang::ast::*;

fn main() {
    let mut comp = Def::new_comp("muladd");
    comp.add_input("a", "i8");
    comp.add_input("b", "i8");
    comp.add_output("y", "i8");
    println!("{}", comp);
}
