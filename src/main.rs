use reticle::frontend::parser::parse_from_file;
use reticle::lang::ast::{Def, Instr, Prog};
use reticle::passes::select::Select;

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
    prog
}

fn test_parser() {
    let prog = parse_from_file("examples/prog.ret");
    println!("parse result\n{}", prog);
}

fn main() {
    let prog = sample_prog();
    let pass = Select::new(prog.clone(), "ultrascale");
    let asm = pass.run();
    let vlog = asm.to_verilog();
    println!("\n\nOriginal program:\n\n{}", &prog);
    println!("\n\nAsm program:\n\n{}", &asm);
    println!("\n\nVerilog module:\n\n{}", vlog);
    test_parser()
}
