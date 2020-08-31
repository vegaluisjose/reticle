pub mod analysis;
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
    let mut map: LocMap = LocMap::new();
    for (_, tree) in input_tree.iter() {
        let output = tree_selection(descriptor.clone(), tree.clone());
        map.extend(tree_locgen(output.clone()));
    }
    let sig = input_prog.defs()[0].signature().clone();
    let mut def = Def::new_with_signature(sig);
    let body = input_prog.defs()[0].body().clone();
    for instr in body.iter() {
        if instr.is_std() {
            def.add_instr(instr.clone());
        } else if let Some(loc) = map.get(&instr.dst_id()) {
            let mut instr_with_loc = instr.clone();
            instr_with_loc.set_loc(loc.clone());
            def.add_instr(instr_with_loc);
        }
    }
    let mut output_prog = Prog::default();
    output_prog.add_def(def);
    output_prog
}

pub fn map_analysis(input_prog: Prog) -> analysis::Analysis {
    let mut analysis = analysis::Analysis::default();
    let body = input_prog.defs()[0].body().clone();
    for instr in body.iter() {
        if instr.is_prim() {
            analysis.inc_prim();
            if instr.is_hole() {
                analysis.inc_hole();
            } else if instr.is_lut() {
                analysis.inc_lut();
            } else if instr.is_dsp() {
                analysis.inc_dsp();
            } else if instr.is_lum() {
                analysis.inc_lum();
            } else if instr.is_ram() {
                analysis.inc_ram();
            }
        } else {
            analysis.inc_std();
        }
    }
    analysis
}

pub fn map_clear(input_prog: Prog) -> Prog {
    let sig = input_prog.defs()[0].signature().clone();
    let mut def = Def::new_with_signature(sig);
    let body = input_prog.defs()[0].body().clone();
    for instr in body.iter() {
        if instr.is_prim() {
            let mut instr_clear = instr.clone();
            instr_clear.clear_loc();
            def.add_instr(instr_clear);
        } else {
            def.add_instr(instr.clone());
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
    let mut map: InstrMap = InstrMap::new();
    for (_, tree) in input_tree.iter() {
        let output = tree_selection(descriptor.clone(), tree.clone());
        map.extend(tree_codegen(output.clone()));
    }
    let sig = input_prog.defs()[0].signature().clone();
    let body = input_prog.defs()[0].body().clone();
    let mut output_prog = asm::Prog::new_with_signature(sig);
    for instr in body.iter() {
        if instr.is_std() {
            output_prog.add_instr(asm::Instr::from(instr.clone()));
        } else if let Some(asm_instr) = map.get(&instr.dst_id()) {
            output_prog.add_instr(asm::Instr::from(asm_instr.clone()));
        }
    }
    output_prog
}
