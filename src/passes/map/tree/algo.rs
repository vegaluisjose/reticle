use crate::backend::asm::ast as asm;
use crate::backend::target::{Descriptor, Tile};
use crate::passes::map::tree::{Tree, TreeGraph, TreeIx, TreeNode};
use petgraph::visit::{Bfs, DfsPostOrder};
use petgraph::Direction;
use std::collections::HashMap;
use std::collections::HashSet;

pub type InstrMap = HashMap<String, asm::Instr>;
pub type LocMap = HashMap<String, asm::Loc>;

pub fn tree_node_stack(graph: TreeGraph, start: TreeIx) -> Vec<TreeNode> {
    let mut stack: Vec<TreeNode> = Vec::new();
    let mut visit = Bfs::new(&graph, start);
    while let Some(ix) = visit.next(&graph) {
        if let Some(node) = graph.node_weight(ix) {
            stack.push(node.clone());
        }
    }
    stack
}

pub fn tree_matches_index(pattern: Tree, input: Tree, input_index: TreeIx) -> Vec<TreeIx> {
    let mut update: Vec<TreeIx> = Vec::new();
    let pattern_index = pattern.root_index().unwrap();
    let pstack = tree_node_stack(pattern.graph().clone(), pattern_index);
    let mut pstack_iter = pstack.iter();
    let mut visit = Bfs::new(&input.graph, input_index);
    let mut discard: HashSet<TreeIx> = HashSet::new();
    while let Some(ix) = visit.next(&input.graph) {
        if !discard.contains(&ix) {
            if let Some(pnode) = pstack_iter.next() {
                if !pnode.is_input() {
                    update.push(ix);
                }
                if pnode.is_input() {
                    let childs = input.graph.neighbors_directed(ix, Direction::Outgoing);
                    // discard childs if parent node in pattern is input
                    for c in childs {
                        discard.insert(c);
                    }
                }
            }
        }
        if pstack_iter.len() == 0 {
            break;
        }
    }
    update
}

pub fn tree_match(pattern: Tree, input: Tree, input_index: TreeIx) -> bool {
    let mut is_match: bool = true;
    let pattern_index = pattern.root_index().unwrap();
    let pstack = tree_node_stack(pattern.graph().clone(), pattern_index);
    let mut pstack_iter = pstack.iter();
    let mut visit = Bfs::new(&input.graph, input_index);
    let mut discard: HashSet<TreeIx> = HashSet::new();
    while let Some(ix) = visit.next(&input.graph) {
        if !discard.contains(&ix) {
            if let Some(pnode) = pstack_iter.next() {
                if let Some(inode) = input.graph.node_weight(ix) {
                    if pnode.ty() != inode.ty() {
                        is_match = false;
                    }
                    if !pnode.is_input() && pnode.op() != inode.op() {
                        is_match = false;
                    }
                    if pnode.is_input() {
                        let childs = input.graph.neighbors_directed(ix, Direction::Outgoing);
                        // discard childs if parent node in pattern is input
                        for c in childs {
                            discard.insert(c);
                        }
                    }
                }
            }
        }
        if !is_match || pstack_iter.len() == 0 {
            break;
        }
    }
    is_match && pstack_iter.len() == 0
}

pub fn tree_update(input: Tree, index: TreeIx, tile: Tile) -> Tree {
    let mut output = input;
    if let Some(node) = output.graph.node_weight_mut(index) {
        node.set_cost(tile.pattern.estimate_cost());
        node.set_tile(tile);
    }
    output
}

pub fn tree_reset(pattern: Tree, input: Tree, input_index: TreeIx) -> Tree {
    let mut output = input.clone();
    let matches = tree_matches_index(pattern, input, input_index);
    for index in matches.iter() {
        if let Some(node) = output.graph.node_weight_mut(*index) {
            node.clear_tile();
            node.set_cost(0.0);
        }
    }
    output
}

pub fn tree_selection(descriptor: Descriptor, input: Tree) -> Tree {
    let mut output = input.clone();
    let start = input.root_index().unwrap();
    let mut dfs = DfsPostOrder::new(&input.graph(), start);
    while let Some(ix) = dfs.next(&input.graph) {
        if let Some(node) = input.graph.node_weight(ix) {
            if !node.is_input() {
                for tile in descriptor.tiles.iter() {
                    if tree_match(tile.pattern.clone(), input.clone(), ix) {
                        let pat_cost = tile.pattern.estimate_cost();
                        let cur_cost = input.estimate_cost_from_index(ix);
                        if pat_cost < cur_cost {
                            output = tree_reset(tile.pattern.clone(), output.clone(), ix);
                            output = tree_update(output.clone(), ix, tile.clone());
                        }
                    }
                }
            }
        }
    }
    output
}

pub fn asm_instr_gen(input: Tree, input_index: TreeIx, tile: Tile) -> asm::Instr {
    let mut instr: asm::Instr = tile.instr().clone();
    let pattern = tile.pattern();
    let pattern_index = pattern.root_index().unwrap();
    let pstack = tree_node_stack(pattern.graph().clone(), pattern_index);
    let mut pstack_iter = pstack.iter();
    let mut visit = Bfs::new(&input.graph, input_index);
    let mut discard: HashSet<TreeIx> = HashSet::new();
    let mut params: Vec<asm::Expr> = Vec::new();
    while let Some(ix) = visit.next(&input.graph) {
        if !discard.contains(&ix) {
            if let Some(pnode) = pstack_iter.next() {
                if let Some(inode) = input.graph.node_weight(ix) {
                    if instr.id().is_empty() {
                        // root
                        instr.set_id(&inode.id());
                    } else if pnode.is_input() {
                        // param
                        params.push(asm::Expr::Ref(inode.id(), inode.ty().clone()));
                    }
                    if pnode.is_input() {
                        let childs = input.graph.neighbors_directed(ix, Direction::Outgoing);
                        // discard childs if parent node in pattern is input
                        for c in childs {
                            discard.insert(c);
                        }
                    }
                }
            }
        }
    }
    // bfs is right-to-left, so need to reverse it here
    params.reverse();
    for param in params.iter() {
        instr.add_param(param.clone());
    }
    instr
}

pub fn tree_asm_codegen(input: Tree) -> InstrMap {
    let mut map: InstrMap = InstrMap::new();
    let root_index = input.root_index().unwrap();
    let graph = input.graph.clone();
    let mut visit = DfsPostOrder::new(&graph, root_index);
    while let Some(ix) = visit.next(&graph) {
        if let Some(node) = graph.node_weight(ix) {
            if let Some(tile) = node.tile() {
                let instr = asm_instr_gen(input.clone(), ix, tile.clone());
                map.insert(instr.id(), instr);
            }
        }
    }
    map
}
