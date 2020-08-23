use reticle::frontend::parser::parse_from_file;
use reticle::passes::map::map_asm;

fn main() {
    let prog = parse_from_file("examples/basic/fsm.ret");
    let asm = map_asm(prog.clone());
    println!("\n---reticle---\n{}", prog);
    println!("\n---reticle asm---\n{}", asm);
}
