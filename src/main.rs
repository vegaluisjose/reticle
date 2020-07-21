use reticle::backend::target::ultrascale::Ultrascale;
use reticle::backend::target::Target;
use reticle::lang::ast::{Def, Instr, Prog};
use reticle::passes::select::block::BasicBlock;
use reticle::passes::select::dag::SDag;

fn sample_prog() -> Prog {
    let mut def = Def::new("muladd");
    def.add_input("a", "i8");
    def.add_input("b", "i8");
    def.add_input("c", "i8");
    def.add_input("en", "bool");
    def.add_output("y", "i8");
    def.add_instr(Instr::new_with_args(
        "t0", "i8", "mul", "a", "i8", "b", "i8", "??",
    ));
    def.add_instr(Instr::new_with_args(
        "t1", "i8", "reg", "t0", "i8", "en", "bool", "??",
    ));
    def.add_instr(Instr::new_with_args(
        "y", "i8", "add", "t1", "i8", "c", "i8", "??",
    ));
    let mut prog = Prog::new();
    prog.add_def(def);
    println!("Original program:\n{}", &prog);
    prog
}

fn codegen(prog: &Prog) {
    let target = Ultrascale::new();
    let block = BasicBlock::from(prog.defs[0].clone());
    let mut sdag = SDag::from(block);
    println!("\n{}", sdag);
    sdag.select("y", &target.to_descriptor());
}

fn main() {
    let prog = sample_prog();
    codegen(&prog);
}
