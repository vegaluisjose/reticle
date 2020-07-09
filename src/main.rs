use reticle::lang::ast::*;

fn main() {
    let mut comp = Def::new_comp("muladd");
    comp.add_input("a", "i8");
    comp.add_input("b", "i8");
    comp.add_input("c", "i8");
    comp.add_output("y", "i8");
    comp.add_decl(Decl::new_instr("t", "i8", "mul", "a", "b"));
    comp.add_decl(Decl::new_instr("y", "i8", "add", "t", "c"));
    let mut prog = Prog::new();
    prog.add_def(comp);
    println!("{}", prog);
}
