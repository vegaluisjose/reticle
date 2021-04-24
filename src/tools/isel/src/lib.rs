pub mod errors;
pub mod tree;

use crate::errors::Error;
use crate::tree::helpers::{
    tree_codegen, tree_select, treelist_try_from_prog, treemap_try_from_target_pair,
};
use crate::tree::TreeMap;
use asm::ast as asm;
use ir::ast as ir;
use pat::ast as pat;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use xim::ast as xim;

pub fn deserialize_pat_from_file(prim: &str) -> pat::Target {
    let filename = format!("{}_pat.bin", prim);
    let path = Path::new(env!("OUT_DIR")).join(filename);
    pat::Target::deserialize_from_file(path)
}

pub fn deserialize_imp_from_file(prim: &str) -> xim::Target {
    let filename = format!("{}_xim.bin", prim);
    let path = Path::new(env!("OUT_DIR")).join(filename);
    xim::Target::deserialize_from_file(path)
}

pub fn instrmap_from_prog(prog: &ir::Prog) -> Result<ir::InstrMap, Error> {
    if let Some(main) = prog.get("main") {
        Ok(ir::InstrMap::from(main.clone()))
    } else {
        Err(Error::new_isel_error("main is not vailable"))
    }
}

// TODO: impl try_from after refactoring done
pub fn try_from_ir_prog(prog: &ir::Prog) -> Result<asm::Prog, Error> {
    if let Some(main) = prog.get("main") {
        let lpat = deserialize_pat_from_file("lut");
        let dpat = deserialize_pat_from_file("dsp");
        let limp = deserialize_imp_from_file("lut");
        let dimp = deserialize_imp_from_file("dsp");
        let lmap = treemap_try_from_target_pair(&lpat, &limp)?;
        let dmap = treemap_try_from_target_pair(&dpat, &dimp)?;
        let imap = instrmap_from_prog(prog)?;
        let blks = treelist_try_from_prog(prog)?;
        let blks = tree_select(&blks, &dmap)?;
        let blks = tree_select(&blks, &lmap)?;
        let mut body: Vec<asm::Instr> = Vec::new();
        let mut iset: HashSet<ir::Id> = HashSet::new();
        let tree_map: TreeMap = lmap.into_iter().chain(dmap).collect();
        let pat_map: HashMap<String, pat::Pat> = lpat
            .pat()
            .clone()
            .into_iter()
            .chain(dpat.pat().clone())
            .collect();
        for blk in blks {
            body.extend(tree_codegen(&mut iset, &imap, &blk, &tree_map, &pat_map)?);
        }
        let mut res = asm::Prog::default();
        res.set_sig(main.sig().clone());
        res.set_body(body);
        Ok(res)
    } else {
        Err(Error::new_isel_error("main is not present"))
    }
}
