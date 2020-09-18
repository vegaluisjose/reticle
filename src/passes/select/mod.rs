pub mod analysis;
pub mod dfg;
pub mod tree;

use crate::backend::arch::ultrascale::Ultrascale;
use crate::backend::asm::ast as asm;
use crate::backend::target::Target;
use crate::lang::ast::{Def, Prog};
use crate::passes::select::dfg::Dfg;
use crate::passes::select::tree::algo::{
    tree_codegen, tree_locgen, tree_selection, InstrMap, LocMap,
};
use crate::passes::select::tree::partition::Partition;

pub fn analysis(input_prog: Prog) -> analysis::Analysis {
    let mut analysis = analysis::Analysis::default();
    let body = input_prog.indexed_def(0).body().clone();
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

pub fn locgen(input_prog: Prog) -> Prog {
    let descriptor = Ultrascale::default().to_descriptor();
    let dfg = Dfg::from(input_prog.clone());
    let input_tree = Partition::from(dfg);
    let mut map: LocMap = LocMap::new();
    for (_, tree) in input_tree.iter() {
        let output = tree_selection(descriptor.clone(), tree.clone());
        map.extend(tree_locgen(output.clone()));
    }
    let sig = input_prog.indexed_def(0).signature().clone();
    let mut def = Def::new_with_signature(sig);
    let body = input_prog.indexed_def(0).body().clone();
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

// for testing purposes. The invariant is a program
// should be the same after compiling location and
// clearing them.
pub fn check_pass(input: Prog) {
    let output = locgen(input.clone());
    let reference = input.indexed_def(0).body();
    let generated = output.indexed_def(0).body();
    assert_eq!(
        reference.len(),
        generated.len(),
        "Error: malformed program, number of instructions differ"
    );
    for (r, g) in reference.iter().zip(generated.iter()) {
        if r.is_std() {
            assert_eq!(r, g, "Error: malformed program, std instructions differ");
        } else if r.is_prim() && !r.loc().is_hole() {
            assert_eq!(r, g, "Error: malformed program, prim instructions differ");
        } else {
            assert_eq!(
                r.prim().op(),
                g.prim().op(),
                "Error: malformed program, op in prim instructions differ"
            );
            assert_eq!(
                r.dst(),
                g.dst(),
                "Error: malformed program, dst expr in prim instructions differ"
            );
            assert_eq!(
                r.attrs(),
                g.attrs(),
                "Error: malformed program, attrs in attrs instructions differ"
            );
            assert_eq!(
                r.params(),
                g.params(),
                "Error: malformed program, params in prim instructions differ"
            );
        }
    }
}

pub fn asmgen(input_prog: Prog, check: bool) -> asm::Prog {
    if check {
        check_pass(input_prog.clone());
    }
    let descriptor = Ultrascale::default().to_descriptor();
    let dfg = Dfg::from(input_prog.clone());
    let input_tree = Partition::from(dfg);
    let mut map: InstrMap = InstrMap::new();
    for (_, tree) in input_tree.iter() {
        let output = tree_selection(descriptor.clone(), tree.clone());
        map.extend(tree_codegen(output.clone()));
    }
    let sig = input_prog.indexed_def(0).signature().clone();
    let body = input_prog.indexed_def(0).body().clone();
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
