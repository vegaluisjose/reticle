use reticle::backend::verilog::Module;
use reticle::frontend::parser::parse_from_file;
use reticle::passes::map::{map_asm, map_loc};

fn main() {
    let prog = parse_from_file("examples/basic/vadd_const.ret");
    let prog_with_loc = map_loc(prog.clone());
    let asm = map_asm(prog.clone());
    let verilog = Module::from(asm.clone());
    println!("\n---original---\n{}", prog);
    println!("\n---with-loc---\n{}", prog_with_loc);
    println!("\n---asm---\n{}", asm);
    println!("\n---verilog---\n{}", verilog);
}
