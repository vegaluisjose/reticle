pub mod default;
pub mod display;
pub mod from;
pub mod helpers;
pub mod tree;
pub mod try_from;

use crate::asm::ast as asm;
use crate::compiler::tree::*;
use crate::ir::ast as ir;
use crate::tdl::ast::Pat;
use crate::tdl::parser::TDLParser;
use crate::util::errors::Error;
use crate::util::file::create_abs_path;
use std::collections::{HashMap, HashSet};

pub fn select(prog: &ir::Prog) -> Result<asm::Prog, Error> {
    let lut_tdl = create_abs_path("examples/target/ultrascale/lut.tdl");
    let dsp_tdl = create_abs_path("examples/target/ultrascale/dsp.tdl");
    let lut = TDLParser::parse_from_file(lut_tdl)?;
    let dsp = TDLParser::parse_from_file(dsp_tdl)?;
    let lmap = tree_from_pats(lut.pat())?;
    let dmap = tree_from_pats(dsp.pat())?;
    let imap = imap_from_prog(&prog)?;
    let blks = tree_from_prog(&prog)?;
    let blks = tree_select(&blks, &lmap)?;
    let blks = tree_select(&blks, &dmap)?;
    let mut body: Vec<asm::Instr> = Vec::new();
    let mut iset: HashSet<ir::Id> = HashSet::new();
    let tree_map: HashMap<String, Tree> = lmap.into_iter().chain(dmap).collect();
    let pat_map: HashMap<String, Pat> = lut
        .pat()
        .clone()
        .into_iter()
        .chain(dsp.pat().clone())
        .collect();
    for blk in blks {
        body.extend(tree_codegen(&mut iset, &imap, &blk, &tree_map, &pat_map)?);
    }
    let mut res = asm::Prog::default();
    if let Some(main) = prog.get("main") {
        res.set_sig(main.sig().clone());
        res.set_body(body);
        Ok(res)
    } else {
        Err(Error::new_compiler_error(
            "Prog must have a main definition",
        ))
    }
}
