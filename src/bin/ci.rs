use reticle::backend::verilog::Module;
use reticle::frontend::parser::parse_from_file;
use reticle::passes::map::{map_analysis, map_asm, map_clear, map_loc};
use reticle::util::file::write_to_file;

pub fn compile(input: &str, output: &str) {
    let prog = parse_from_file(input);
    let prog_with_loc = map_loc(prog.clone());
    let analysis = map_analysis(prog_with_loc.clone());
    let asm = map_asm(prog.clone());
    let verilog = Module::from(asm);
    assert_eq!(prog, map_clear(prog_with_loc));
    assert_eq!(analysis.num_holes(), 0);
    write_to_file(output, &verilog.to_string());
}

pub fn main() {
    compile("examples/basic/fsm.ret", "ci/fsm.v");
    compile("examples/basic/vadd_const.ret", "ci/vadd_const.v");
    compile("examples/isa/scalar/register.ret", "ci/register.v");
}
