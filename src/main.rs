use reticle::frontend::parser::parse_from_file;
use reticle::lang::interp::trace::Trace;
use reticle::lang::interp::Interpreter;

fn test_identity() {
    let prog = parse_from_file("examples/identity.ret");
    let mut trace = Trace::default();
    trace.enq("a", 9);
    trace.enq("a", 3);
    trace.enq("y", 9);
    trace.enq("y", 3);
    assert!(!Interpreter::default()
        .with_print()
        .run(&prog, &trace)
        .is_failed());
}

fn main() {
    test_identity();
}

// compilation pipeline example
// use reticle::frontend::type_check::type_check;
// use reticle::passes::select::Select;
// fn main() {
//     let prog = parse_from_file("examples/muladd.ret");
//     let typed_prog = type_check(&prog);
//     let pass = Select::new(typed_prog.clone(), "ultrascale");
//     let asm = pass.run();
//     let vlog = asm.to_verilog();
//     println!("\n\nOriginal program:\n\n{}", &typed_prog);
//     println!("\n\nAsm program:\n\n{}", &asm);
//     println!("\n\nVerilog module:\n\n{}", vlog);
// }
