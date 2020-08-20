pub mod dag;
pub mod partition;

use crate::backend::arch::ultrascale::Ultrascale;
use crate::backend::target::{Descriptor, Target};
use crate::lang::ast::Prog;
use crate::passes::map::dag::Dag;
use crate::passes::map::partition::tree::{Tree, TreeGraph, TreeIx, TreeNode};
use crate::passes::map::partition::Partition;
use petgraph::visit::Bfs;

fn tree_stack(graph: TreeGraph, start: TreeIx) -> Vec<TreeNode> {
    let mut stack: Vec<TreeNode> = Vec::new();
    let mut bfs = Bfs::new(&graph, start);
    while let Some(ix) = bfs.next(&graph) {
        if let Some(node) = graph.node_weight(ix) {
            stack.push(node.clone());
        }
    }
    stack
}

fn tree_match(pattern: Tree, input: Tree, input_index: TreeIx) -> bool {
    let pattern_index = *pattern.root_index().unwrap();
    let pstack = tree_stack(pattern.graph().clone(), pattern_index);
    let istack = tree_stack(input.graph().clone(), input_index);
    if pstack.len() != istack.len() {
        false
    } else {
        let mut matched = true;
        for (p, i) in pstack.iter().zip(istack.iter()) {
            if p.ty() != i.ty() || !p.is_input() && p.op() != i.op() {
                matched = false;
                break;
            }
        }
        matched
    }
}

pub fn example(prog: Prog) {
    let descriptor = Ultrascale::default().to_descriptor();
    let dag = Dag::from(prog);
    let partition = Partition::from(dag);
    for (_, tree) in partition.iter() {
        let pattern = descriptor.tiles[0].pattern.clone();
        let index = *tree.root_index().unwrap();
        println!(
            "new tree, match:{}",
            tree_match(pattern, tree.clone(), index)
        );
    }
}
