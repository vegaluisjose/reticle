use reticle::lang::ast::*;
use reticle::passes::select::dag::DAG;

fn main() {
    let mut comp = Def::new_comp("muladd");
    comp.add_input("a", "i8");
    comp.add_input("b", "i8");
    comp.add_input("c", "i8");
    comp.add_input("en", "bool");
    comp.add_output("z", "i8");
    comp.add_decl(Decl::new_instr("x", "i8", "mul", "a", "b"));
    comp.add_decl(Decl::new_instr("y", "i8", "reg", "x", "en"));
    comp.add_decl(Decl::new_instr("z", "i8", "add", "y", "c"));
    let mut prog = Prog::new();
    prog.add_def(comp);
    println!("{}", prog);
    let mut dag = DAG::new();
    dag.from_prog(&prog);
}
