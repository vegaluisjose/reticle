use reticle::backend::asm::ast as asm;
use reticle::backend::verilog::Module;
use reticle::frontend::parser::parse_from_file;
use reticle::util::file::write_to_file;

pub fn compile(input: &str, output: &str) {
    let prog = parse_from_file(input);
    let asm = asm::Prog::from(prog);
    let verilog = Module::from(asm);
    write_to_file(output, &verilog.to_string());
}

pub fn main() {
    compile("examples/basic/fsm.ret", "ci/fsm.v");
    compile("examples/basic/vadd_const.ret", "ci/vadd_const.v");
    compile("examples/isa/scalar/register.ret", "ci/register.v");
}
