pub fn compiler_example() {
    use reticle::backend::asm::verilog::ToVerilog;
    use reticle::frontend::parser::parse_from_file;
    use reticle::passes::map::{map_analysis, map_asm, map_clear, map_loc};
    let prog = parse_from_file("examples/basic/fsm.ret");
    let prog_with_loc = map_loc(prog.clone());
    let analysis = map_analysis(prog_with_loc.clone());
    let asm = map_asm(prog.clone());
    let verilog = asm.to_verilog();
    assert_eq!(prog, map_clear(prog_with_loc.clone()));
    assert_eq!(analysis.num_holes(), 0);
    println!("\n---reticle---\n{}", prog);
    println!("\n---reticle with loc---\n{}", prog_with_loc);
    println!("\n---reticle asm---\n{}", asm);
    println!("\n---verilog---\n{}", verilog);
}

pub fn lut_example() {
    use reticle::backend::arch::ultrascale::lut::Lut;
    let mut lut2 = Lut::new_lut2("i0");
    lut2.add_input("a0");
    lut2.add_input("b0");
    lut2.set_output("y");
    lut2.set_init(6);
    println!("\n{}", lut2);
}

pub fn reg_example() {
    use reticle::backend::arch::ultrascale::reg::Reg;
    let mut reg = Reg::new_fdre("r0");
    reg.set_clock("clock");
    reg.set_reset("reset");
    reg.set_en("en");
    reg.set_input("a");
    reg.set_output("y");
    println!("\n{}", reg);
}

fn main() {
    compiler_example();
    lut_example();
    reg_example();
}
