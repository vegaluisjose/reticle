//// compilation pipeline example
//use reticle::frontend::parser::parse_from_file;
//use reticle::frontend::type_check::type_check;
//use reticle::passes::select::Select;
//fn main() {
//    let prog = parse_from_file("examples/basic/muladd.ret");
//    let typed_prog = type_check(&prog);
//    let pass = Select::new(typed_prog.clone(), "ultrascale");
//    let asm = pass.run();
//    let vlog = asm.to_verilog();
//    println!("\n\nOriginal program:\n\n{}", &typed_prog);
//    println!("\n\nAsm program:\n\n{}", &asm);
//    println!("\n\nVerilog module:\n\n{}", vlog);
//}

use reticle::lang::dot::example;

fn main() {
    example();
}
