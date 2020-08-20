pub mod dag;
pub mod partition;

use crate::backend::arch::ultrascale::Ultrascale;
use crate::backend::target::{Descriptor, Target};
use crate::lang::ast::Prog;
use crate::passes::map::dag::Dag;
use crate::passes::map::partition::tree::{Tree, TreeGraph, TreeIx, TreeNode};
use crate::passes::map::partition::Partition;
use petgraph::visit::{Dfs, DfsPostOrder};

fn tree_stack(graph: TreeGraph, start: TreeIx) -> Vec<TreeNode> {
    let mut stack: Vec<TreeNode> = Vec::new();
    let mut visit = Dfs::new(&graph, start);
    while let Some(ix) = visit.next(&graph) {
        if let Some(node) = graph.node_weight(ix) {
            stack.push(node.clone());
        }
    }
    stack
}

fn tree_match(pattern: Tree, input: Tree, input_index: TreeIx) -> bool {
    let pattern_index = pattern.root_index().unwrap();
    let pstack = tree_stack(pattern.graph().clone(), pattern_index);
    let istack = tree_stack(input.graph().clone(), input_index);
    if istack.len() < pstack.len() {
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

fn select(descriptor: Descriptor, input: Tree) {
    let start = input.root_index().unwrap();
    let mut dfs = DfsPostOrder::new(&input.graph(), start);
    while let Some(ix) = dfs.next(&input.graph) {
        if let Some(node) = input.graph.node_weight(ix) {
            if !node.is_input() {
                println!("\nchecking node:{}", node);
                for tile in descriptor.tiles.iter() {
                    if tree_match(tile.pattern.clone(), input.clone(), ix) {
                        println!("match with node:{} ~~with~~ pattern:{}", &node, &tile.instr);
                    }
                }
            }
        }
    }
}

pub fn example(prog: Prog) {
    let descriptor = Ultrascale::default().to_descriptor();
    let dag = Dag::from(prog);
    let partition = Partition::from(dag);
    for (_, tree) in partition.iter() {
        println!("\n{}", tree);
        select(descriptor.clone(), tree.clone());
    }
}
