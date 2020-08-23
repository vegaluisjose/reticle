use reticle::frontend::parser::parse_from_file;
use reticle::passes::map;

fn main() {
    let prog = parse_from_file("examples/basic/fsm.ret");
    map::example(prog.clone());
    println!("\n{}", prog);
}
