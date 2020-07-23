use reticle::backend::asm::ast as asm;
use reticle::backend::target::ultrascale::Ultrascale;
use reticle::backend::target::Target;
use reticle::lang::ast::{Def, Instr, Prog};
use reticle::passes::select::block::BasicBlock;
use reticle::passes::select::sdag::SDag;

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
    println!("Original program:\n\n{}", &prog);
    prog
}

fn compile(prog: &Prog) {
    let target = Ultrascale::new();
    let block = BasicBlock::from(prog.defs[0].clone());
    let mut sdag = SDag::from(block);
    sdag.select_mut("y", &target.to_descriptor());
    let asm_instr = sdag.codegen("y");
    let mut asm_prog = asm::Prog::new(prog.defs[0].sig.clone());
    for instr in asm_instr.iter() {
        asm_prog.add_instr(instr.clone());
    }
    println!("Assembly program:\n\n{}", asm_prog);
}

fn main() {
    let prog = sample_prog();
    compile(&prog);
}
