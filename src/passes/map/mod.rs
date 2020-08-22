pub mod dag;
pub mod tree;

use crate::backend::arch::ultrascale::Ultrascale;
use crate::backend::target::Target;
use crate::lang::ast::Prog;
use crate::passes::map::dag::Dag;
use crate::passes::map::tree::algo::{tree_codegen, tree_selection};
use crate::passes::map::tree::partition::Partition;

pub fn example(prog: Prog) {
    let descriptor = Ultrascale::default().to_descriptor();
    let dag = Dag::from(prog);
    let input = Partition::from(dag);
    let mut output = Partition::new();
    for (id, tree) in input.iter() {
        println!("\n{}", tree);
        output.insert(
            id.to_string(),
            tree_selection(descriptor.clone(), tree.clone()),
        );
    }
    for (id, tree) in output.iter() {
        println!("\ncodegen ---> {}", id);
        tree_codegen(tree.clone());
    }
}
