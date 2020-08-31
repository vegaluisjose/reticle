use reticle::frontend::parser::parse_from_file;
use reticle::passes::map::{map_analysis, map_asm, map_loc};

fn main() {
    let prog = parse_from_file("examples/isa/scalar/reg.ret");
    let prog_with_loc = map_loc(prog.clone());
    let analysis = map_analysis(prog_with_loc.clone());
    let asm = map_asm(prog.clone());
    println!("\n{}\n", prog);
    println!("\n{}\n", prog_with_loc);
    println!("\n{}\n", asm);
    println!("\n{}\n", analysis);
}
