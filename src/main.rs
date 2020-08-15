use reticle::frontend::parser::parse_from_file;
use reticle::passes::map::example;

fn main() {
    let prog = parse_from_file("examples/basic/fsm.ret");
    example(&prog);
}
