pub mod dag;
pub mod tree;

use crate::backend::arch::ultrascale::Ultrascale;
use crate::backend::asm::ast as asm;
use crate::backend::target::Target;
use crate::lang::ast::{Def, Prog};
use crate::passes::map::dag::Dag;
use crate::passes::map::tree::algo::{
    tree_asm_codegen, tree_locgen, tree_selection, InstrMap, LocMap,
};
use crate::passes::map::tree::partition::Partition;

pub fn map_loc(prog_input: Prog) -> Prog {
    let descriptor = Ultrascale::default().to_descriptor();
    let dag = Dag::from(prog_input.clone());
    let tree_input = Partition::from(dag);
    let mut tree_output = Partition::new();
    for (id, tree) in tree_input.iter() {
        tree_output.insert(
            id.to_string(),
            tree_selection(descriptor.clone(), tree.clone()),
        );
    }
    let mut map: LocMap = LocMap::new();
    for (_, tree) in tree_output.iter() {
        map.extend(tree_locgen(tree.clone()));
    }
    let sig = prog_input.defs()[0].signature().clone();
    let mut def = Def::new_with_signature(sig);
    let body = prog_input.defs()[0].body().clone();
    for instr in body.iter() {
        if instr.is_std() {
            def.add_instr(instr.clone());
        } else if let Some(loc) = map.get(&instr.id()) {
            let mut instr_with_loc = instr.clone();
            instr_with_loc.set_loc(loc.clone());
            def.add_instr(instr_with_loc);
        }
    }
    let mut prog_output = Prog::default();
    prog_output.add_def(def);
    prog_output
}

pub fn map_asm(prog: Prog) -> asm::Prog {
    let descriptor = Ultrascale::default().to_descriptor();
    let dag = Dag::from(prog.clone());
    let input = Partition::from(dag);
    let mut output = Partition::new();
    for (id, tree) in input.iter() {
        output.insert(
            id.to_string(),
            tree_selection(descriptor.clone(), tree.clone()),
        );
    }
    let mut map: InstrMap = InstrMap::new();
    for (_, tree) in output.iter() {
        map.extend(tree_asm_codegen(tree.clone()));
    }
    let sig = prog.defs()[0].signature().clone();
    let body = prog.defs()[0].body().clone();
    let mut asm = asm::Prog::new_with_signature(sig);
    for instr in body.iter() {
        if instr.is_std() {
            asm.add_instr(asm::Instr::from(instr.clone()));
        } else if let Some(asm_instr) = map.get(&instr.id()) {
            asm.add_instr(asm_instr.clone());
        }
    }
    asm
}
