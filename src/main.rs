use reticle::backend::ultrascale;
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
    def.add_instr(Instr::new_with_args("t0", "i8", "mul", "a", "b", "??"));
    def.add_instr(Instr::new_with_args("t1", "i8", "reg", "t0", "en", "??"));
    def.add_instr(Instr::new_with_args("y", "i8", "add", "t1", "c", "??"));
    let mut prog = Prog::new();
    prog.add_def(def);
    println!("Original program:\n{}", &prog);
    prog
}

fn target_info() {
    let target = ultrascale::target();
    println!("\nultrascale\n");
    for tile in target.tiles.iter() {
        println!("[pattern] name:{} cost:{}", tile.pattern.name, tile.pattern.cost);
        for i in tile.pattern.instr.iter() {
            println!("    instr:{}", i);
        }
        println!("[asm] instr:{:?}", tile.asm);
    }
}

fn create_dag_from_prog(prog: &Prog) {
    let block = BasicBlock::from(prog.defs[0].clone());
    let sdag = SDag::from(block);
    println!("\n{}", sdag);
}

fn main() {
    let prog = sample_prog();
    target_info();
    create_dag_from_prog(&prog);
}
