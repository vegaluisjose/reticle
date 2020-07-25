use handlebars::Handlebars;
use reticle::backend::asm::ast as asm;
use reticle::backend::target::ultrascale::Ultrascale;
use reticle::backend::target::Target;
use reticle::backend::verilog::Module;
use reticle::lang::ast::{Def, Instr, Prog};
use reticle::passes::select::basic_block::BasicBlock;
use reticle::passes::select::sdag::SDag;
use reticle::util::file::read_to_string;
use serde_json::json;

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
    let mut prog = Prog::default();
    prog.add_def(def);
    println!("\n\nOriginal program:\n\n{}", &prog);
    prog
}

fn compile(prog: &Prog) -> asm::Prog {
    let target = Ultrascale::default();
    let block = BasicBlock::from(prog.defs[0].clone());
    let mut sdag = SDag::from(block);
    sdag.select_mut("y", &target.to_descriptor());
    let asm_instr = sdag.codegen("y");
    let mut asm_prog = asm::Prog::new(prog.defs[0].sig.clone());
    for instr in asm_instr.iter() {
        asm_prog.add_instr(instr.clone());
    }
    println!("\n\nAssembly program:\n\n{}", asm_prog);
    asm_prog
}

fn test_dsp_prim() {
    let reg = Handlebars::new();
    let dsp = read_to_string("spec/ultrascale/dsp.hb");
    let render = reg
        .render_template(&dsp, &json!({"name": "foo"}))
        .expect("Error: rendering handlebars");
    println!("\n\n{}", render);
}

fn main() {
    let prog = sample_prog();
    let asm = compile(&prog);
    let vlog = Module::from(asm);
    println!("\n\nVerilog module:\n\n{}", vlog);
    test_dsp_prim();
}
