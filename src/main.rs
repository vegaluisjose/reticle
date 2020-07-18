use reticle::backend::ultrascale;
use reticle::lang::ast::{Def, Instr, Prog};
use reticle::passes::select::dag::SDag;

fn sample_prog() -> Prog {
    let mut def = Def::new("muladd");
    def.add_input("a", "i8");
    def.add_input("b", "i8");
    def.add_input("c", "i8");
    def.add_input("en", "bool");
    def.add_output("z", "i8");
    def.add_instr(Instr::new_with_args("x", "i8", "mul", "a", "b", "??"));
    def.add_instr(Instr::new_with_args("y", "i8", "reg", "x", "en", "??"));
    def.add_instr(Instr::new_with_args("z", "i8", "add", "y", "c", "??"));
    let mut prog = Prog::new();
    prog.add_def(def);
    prog
}

fn main() {
    let prog = sample_prog();
    println!("Original program:\n{}", prog);
    let target = ultrascale::target();
    println!("ultrascale patterns\n");
    for p in target.patterns.iter() {
        println!("name:{} cost:{}", p.name, p.cost);
        for i in p.instr.iter() {
            println!("    instr:{}", i);
        }
    }
    let dag = SDag::from(prog.clone());
}
