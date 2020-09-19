use crate::backend::asm::ast as asm;
use crate::backend::target::{Descriptor, Tile};
use crate::passes::select::tree::{Tree, TreeGraph, TreeIx, TreeNode};
use petgraph::visit::{Bfs, DfsPostOrder};
use petgraph::Direction;
use std::collections::HashMap;
use std::collections::HashSet;

pub type InstrMap = HashMap<String, asm::InstrPrim>;
pub type LocMap = HashMap<String, asm::LocTy>;

pub fn tree_node_stack(graph: &TreeGraph, start: TreeIx) -> Vec<TreeNode> {
    let mut stack: Vec<TreeNode> = Vec::new();
    let mut visit = Bfs::new(graph, start);
    while let Some(ix) = visit.next(&graph) {
        if let Some(node) = graph.node_weight(ix) {
            stack.push(node.clone());
        }
    }
    stack
}

pub fn tree_index_stack(graph: &TreeGraph, start: TreeIx) -> Vec<TreeIx> {
    let mut stack: Vec<TreeIx> = Vec::new();
    let mut visit = Bfs::new(graph, start);
    while let Some(ix) = visit.next(&graph) {
        stack.push(ix);
    }
    stack
}

pub fn tree_matches_index(pattern: &Tree, input: Tree, input_index: TreeIx) -> Vec<TreeIx> {
    let mut update: Vec<TreeIx> = Vec::new();
    let pindex = pattern.root_index().unwrap();
    let pgraph = pattern.graph();
    let pstack = tree_node_stack(pgraph, pindex);
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

pub fn tree_match(tile: &Tile, pattern_index: TreeIx, input: &Tree, input_index: TreeIx) -> bool {
    let mut is_match: bool = true;
    let pgraph = tile.pattern().graph();
    let pstack = tree_index_stack(pgraph, pattern_index);
    let mut pstack_iter = pstack.iter();
    let mut visit = Bfs::new(&input.graph, input_index);
    let mut discard: HashSet<TreeIx> = HashSet::new();
    while let Some(ix) = visit.next(&input.graph) {
        if !discard.contains(&ix) {
            if let Some(px) = pstack_iter.next() {
                if let Some(pnode) = tile.pattern().graph().node_weight(*px) {
                    if let Some(inode) = input.graph.node_weight(ix) {
                        if pnode.ty() != inode.ty() {
                            is_match = false;
                        }
                        if !pnode.is_input() && pnode.op() != inode.op() {
                            is_match = false;
                        }
                        if !inode.loc().is_hole() && inode.loc() != tile.loc() {
                            is_match = false;
                        }
                        if pnode.is_input() {
                            let childs = input.graph.neighbors_directed(ix, Direction::Outgoing);
                            // discard childs if parent node in pattern is input
                            for c in childs {
                                discard.insert(c);
                            }
                        } else if is_match {
                            if let Some(prev) = inode.tile() {
                                let prev_tree = prev.pattern();
                                let prev_index = prev.pattern().root_index().unwrap();
                                is_match &= tree_match(tile, *px, prev_tree, prev_index);
                            }
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

pub fn tree_reset(pattern: Tree, input: Tree, input_index: TreeIx) -> Tree {
    let mut output = input.clone();
    let matches = tree_matches_index(&pattern, input, input_index);
    for index in matches.iter() {
        if let Some(node) = output.graph.node_weight_mut(*index) {
            node.clear_tile();
            node.set_cost(0.0);
        }
    }
    output
}

pub fn tree_update(tile: &Tile, input: Tree, index: TreeIx) -> Tree {
    let mut output = input;
    if let Some(node) = output.graph.node_weight_mut(index) {
        node.set_cost(tile.pattern().estimate_cost());
        node.set_tile(tile.clone());
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
                    let pattern_index = tile.pattern().root_index().unwrap();
                    let is_match = tree_match(tile, pattern_index, &output, ix);
                    if is_match {
                        let pat_cost = tile.pattern.estimate_cost();
                        let cur_cost = output.estimate_cost_from_index(ix);
                        if pat_cost < cur_cost {
                            output = tree_reset(tile.pattern.clone(), output.clone(), ix);
                            output = tree_update(tile, output.clone(), ix);
                        }
                    }
                }
            }
        }
    }
    output
}

pub fn subtree_codegen(tile: &Tile, input: Tree, input_index: TreeIx) -> asm::InstrPrim {
    let mut instr: asm::InstrPrim = tile.instr().clone();
    let pattern = tile.pattern();
    let pindex = pattern.root_index().unwrap();
    let pgraph = pattern.graph();
    let pstack = tree_node_stack(pgraph, pindex);
    let mut pstack_iter = pstack.iter();
    let mut visit = Bfs::new(&input.graph, input_index);
    let mut discard: HashSet<TreeIx> = HashSet::new();
    let mut params: Vec<asm::Expr> = Vec::new();
    let mut attrs: Vec<asm::Expr> = Vec::new();
    while let Some(ix) = visit.next(&input.graph) {
        if !discard.contains(&ix) {
            if let Some(pnode) = pstack_iter.next() {
                if let Some(inode) = input.graph.node_weight(ix) {
                    attrs.extend(inode.attrs().clone());
                    if instr.dst_id().is_empty() {
                        // root
                        instr.set_dst_id(&inode.id());
                    } else if pnode.is_input() {
                        // param
                        params.push(asm::Expr::new_ref(&inode.id(), inode.ty().clone()));
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
    attrs.reverse();
    params.reverse();
    for attr in attrs.iter() {
        instr.add_attr(attr.clone());
    }
    for param in params.iter() {
        instr.add_param(param.clone());
    }
    instr
}

pub fn tree_codegen(input: Tree) -> InstrMap {
    let mut map: InstrMap = InstrMap::new();
    let root_index = input.root_index().unwrap();
    let graph = input.graph.clone();
    let mut visit = DfsPostOrder::new(&graph, root_index);
    while let Some(ix) = visit.next(&graph) {
        if let Some(node) = graph.node_weight(ix) {
            if let Some(tile) = node.tile() {
                let instr = subtree_codegen(tile, input.clone(), ix);
                map.insert(instr.dst_id(), instr);
            }
        }
    }
    map
}

pub fn subtree_locgen(tile: &Tile, input: Tree, input_index: TreeIx) -> LocMap {
    let loc = tile.loc().clone();
    let pattern = tile.pattern();
    let pindex = pattern.root_index().unwrap();
    let pgraph = pattern.graph();
    let pstack = tree_node_stack(pgraph, pindex);
    let mut pstack_iter = pstack.iter();
    let mut visit = Bfs::new(&input.graph, input_index);
    let mut discard: HashSet<TreeIx> = HashSet::new();
    let mut map = LocMap::new();
    while let Some(ix) = visit.next(&input.graph) {
        if !discard.contains(&ix) {
            if let Some(pnode) = pstack_iter.next() {
                if let Some(inode) = input.graph.node_weight(ix) {
                    if !pnode.is_input() {
                        map.insert(inode.id(), loc.clone());
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
    map
}

pub fn tree_locgen(input: Tree) -> LocMap {
    let mut map: LocMap = LocMap::new();
    let root_index = input.root_index().unwrap();
    let graph = input.graph.clone();
    let mut visit = DfsPostOrder::new(&graph, root_index);
    while let Some(ix) = visit.next(&graph) {
        if let Some(node) = graph.node_weight(ix) {
            if let Some(tile) = node.tile() {
                map.extend(subtree_locgen(tile, input.clone(), ix));
            }
        }
    }
    map
}
