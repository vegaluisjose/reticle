// use reticle::frontend::parser::parse_from_file;
// use reticle::passes::map::{map_analysis, map_asm, map_clear, map_loc};

// fn main() {
//     let prog = parse_from_file("examples/basic/fsm.ret");
//     let prog_with_loc = map_loc(prog.clone());
//     let analysis = map_analysis(prog_with_loc.clone());
//     let asm = map_asm(prog.clone());
//     assert_eq!(prog, map_clear(prog_with_loc.clone()));
//     assert_eq!(analysis.num_holes(), 0);
//     println!("\n---reticle---\n{}", prog);
//     println!("\n---reticle with loc---\n{}", prog_with_loc);
//     println!("\n---reticle asm---\n{}", asm);
//     println!("\n---Analysis---{}", analysis);
// }

use reticle::backend::arch::ultrascale::prim::lut::Lut;

pub fn main() {
    let lut2 = Lut::new_lut2("i0");
    println!("this is a lut ---> {}", lut2);
}
