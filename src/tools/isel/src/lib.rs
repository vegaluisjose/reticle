pub mod errors;
pub mod tree;

use crate::errors::Error;
use asm::ast as asm;
use ir::ast as ir;
use pat::ast as pat;
use std::path::Path;

pub fn tree_from_pat_target(prim: &str) {
    let filename = format!("{}_pat.bin", prim);
    let path = Path::new(env!("OUT_DIR")).join(filename);
    let target = pat::Target::deserialize_from_file(path);
    println!("{}", target);
}

pub fn select(_: &ir::Prog) -> Result<asm::Prog, Error> {
    tree_from_pat_target("lut");
    let prog = asm::Prog::default();
    Ok(prog)
}
