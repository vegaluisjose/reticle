pub mod dag;
pub mod tree;

use crate::backend::arch::ultrascale::Ultrascale;
use crate::backend::asm::ast as asm;
use crate::backend::target::Target;
use crate::lang::ast::{Def, Prog};
use crate::passes::map::dag::Dag;
use crate::passes::map::tree::algo::{tree_codegen, tree_locgen, tree_selection, InstrMap, LocMap};
use crate::passes::map::tree::partition::Partition;

pub fn map_loc(input_prog: Prog) -> Prog {
    let descriptor = Ultrascale::default().to_descriptor();
    let dag = Dag::from(input_prog.clone());
    let input_tree = Partition::from(dag);
    let mut output_tree = Partition::new();
    for (id, tree) in input_tree.iter() {
        output_tree.insert(
            id.to_string(),
            tree_selection(descriptor.clone(), tree.clone()),
        );
    }
    let mut map: LocMap = LocMap::new();
    for (_, tree) in output_tree.iter() {
        map.extend(tree_locgen(tree.clone()));
    }
    let sig = input_prog.defs()[0].signature().clone();
    let mut def = Def::new_with_signature(sig);
    let body = input_prog.defs()[0].body().clone();
    for instr in body.iter() {
        if instr.is_std() {
            def.add_instr(instr.clone());
        } else if let Some(loc) = map.get(&instr.id()) {
            let mut instr_with_loc = instr.clone();
            instr_with_loc.set_loc(loc.clone());
            def.add_instr(instr_with_loc);
        }
    }
    let mut output_prog = Prog::default();
    output_prog.add_def(def);
    output_prog
}

pub fn map_asm(input_prog: Prog) -> asm::Prog {
    let descriptor = Ultrascale::default().to_descriptor();
    let dag = Dag::from(input_prog.clone());
    let input_tree = Partition::from(dag);
    let mut output_tree = Partition::new();
    for (id, tree) in input_tree.iter() {
        output_tree.insert(
            id.to_string(),
            tree_selection(descriptor.clone(), tree.clone()),
        );
    }
    let mut map: InstrMap = InstrMap::new();
    for (_, tree) in output_tree.iter() {
        map.extend(tree_codegen(tree.clone()));
    }
    let sig = input_prog.defs()[0].signature().clone();
    let body = input_prog.defs()[0].body().clone();
    let mut output_prog = asm::Prog::new_with_signature(sig);
    for instr in body.iter() {
        if instr.is_std() {
            output_prog.add_instr(asm::Instr::from(instr.clone()));
        } else if let Some(asm_instr) = map.get(&instr.id()) {
            output_prog.add_instr(asm_instr.clone());
        }
    }
    output_prog
}
