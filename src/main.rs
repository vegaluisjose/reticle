use reticle::frontend::parser::parse_from_file;
use reticle::lang::ast::Prog;
use reticle::lang::interp::eval::Eval;
use reticle::lang::interp::state::State;

fn test_identity(prog: &Prog) {
    let mut state = State::default();
    state.add_input("a", 4);
    for def in prog.defs().iter() {
        for instr in def.body().iter() {
            println!("{}", instr.eval_current(&state));
        }
    }
}

fn main() {
    let prog = parse_from_file("examples/identity.ret");
    println!("{}", &prog);
    test_identity(&prog);
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
