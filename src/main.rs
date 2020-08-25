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
    let mut lut2 = Lut::new_lut2("i0");
    lut2.add_input("a0");
    lut2.add_input("b0");
    lut2.set_output("y");
    lut2.set_init(6);
    println!("{}", lut2);
}
