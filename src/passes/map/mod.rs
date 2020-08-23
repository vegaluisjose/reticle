pub mod dag;
pub mod tree;

use crate::backend::arch::ultrascale::Ultrascale;
use crate::backend::asm::ast as asm;
use crate::backend::target::Target;
use crate::lang::ast::Prog;
use crate::passes::map::dag::Dag;
use crate::passes::map::tree::algo::{tree_asm_codegen, tree_selection};
use crate::passes::map::tree::partition::Partition;
use std::collections::HashMap;

pub type AsmInstrMap = HashMap<String, asm::Instr>;

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
    let mut map: AsmInstrMap = AsmInstrMap::new();
    for (_, tree) in output.iter() {
        map.extend(tree_asm_codegen(tree.clone()));
    }
    let sig = prog.defs()[0].signature();
    asm::Prog::new_with_signature(sig.clone())
}
