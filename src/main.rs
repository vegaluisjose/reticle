// compilation pipeline example
use reticle::frontend::parser::parse_from_file;
use reticle::passes::select::Select;
fn main() {
    let prog = parse_from_file("examples/basic/muladd.ret");
    let pass = Select::new(prog.clone(), "ultrascale");
    let asm = pass.run();
    let vlog = asm.to_verilog();
    println!("\n\nOriginal program:\n\n{}", &prog);
    println!("\n\nAsm program:\n\n{}", &asm);
    println!("\n\nVerilog module:\n\n{}", vlog);
}
