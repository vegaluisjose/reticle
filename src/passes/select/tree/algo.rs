use crate::asm::ast as asm;
use crate::backend::target::{Descriptor, Tile};
use crate::passes::select::tree::{Tree, TreeGraph, TreeIx, TreeNode};
use petgraph::visit::{Dfs, DfsPostOrder};
use petgraph::Direction;
use std::collections::HashMap;
use std::collections::HashSet;

pub type InstrMap = HashMap<String, asm::InstrPrim>;
pub type LocMap = HashMap<String, asm::LocTy>;

pub fn tree_node_stack(graph: &TreeGraph, start: TreeIx) -> Vec<TreeNode> {
    let mut stack: Vec<TreeNode> = Vec::new();
    let mut visit = Dfs::new(graph, start);
    while let Some(ix) = visit.next(&graph) {
        if let Some(node) = graph.node_weight(ix) {
            stack.push(node.clone());
        }
    }
    stack
}

pub fn tree_match(tile: &Tile, input: &Tree, index: TreeIx) -> (bool, f32) {
    let mut is_match: bool = true;
    let root = tile.pattern().root_index().unwrap();
    let graph = tile.pattern().graph();
    let mut stack = tree_node_stack(graph, root);
    stack.reverse();
    let mut visit = Dfs::new(&input.graph, index);
    let mut discard: HashSet<TreeIx> = HashSet::new();
    let mut cost: f32 = 0.0;
    while let Some(ix) = visit.next(&input.graph) {
        if !discard.contains(&ix) {
            if let Some(pnode) = stack.pop() {
                if let Some(inode) = input.graph.node_weight(ix) {
                    if pnode.ty() != inode.ty() {
                        is_match = false;
                    }
                    if !pnode.is_input() && !inode.is_input() && pnode.op() != inode.op() {
                        is_match = false;
                    }
                    if !inode.loc().is_hole() && inode.loc() != tile.loc() {
                        is_match = false;
                    }
                    if pnode.is_input() {
                        let childs = input.graph.neighbors_directed(ix, Direction::Outgoing);
                        for c in childs {
                            discard.insert(c);
                        }
                    } else if is_match {
                        if let Some(prev) = inode.tile() {
                            let prev_root = prev.pattern().root_index().unwrap();
                            let prev_graph = prev.pattern().graph();
                            let mut prev_stack = tree_node_stack(prev_graph, prev_root);
                            prev_stack.reverse();
                            prev_stack.pop();
                            if stack.len() <= prev_stack.len() {
                                for a in prev_stack.iter() {
                                    if let Some(b) = stack.pop() {
                                        visit.next(&input.graph);
                                        if a.op() != b.op() || a.ty() != b.ty() {
                                            is_match = false;
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    cost += inode.cost();
                }
            }
        } else {
            let childs = input.graph.neighbors_directed(ix, Direction::Outgoing);
            for c in childs {
                discard.insert(c);
            }
        }
        if !is_match || stack.is_empty() {
            break;
        }
    }
    (is_match && stack.is_empty(), cost)
}

pub fn tree_update(tile: &Tile, input: &Tree, index: TreeIx) -> Tree {
    let mut output = input.clone();
    let root = tile.pattern().root_index().unwrap();
    let graph = tile.pattern().graph();
    let mut stack = tree_node_stack(graph, root);
    stack.reverse();
    let mut visit = Dfs::new(&input.graph(), index);
    let mut discard: HashSet<TreeIx> = HashSet::new();
    let mut instr = tile.instr().clone();
    let mut params: Vec<asm::Expr> = Vec::new();
    let mut attrs: Vec<asm::Expr> = Vec::new();
    while let Some(ix) = visit.next(&input.graph) {
        if !discard.contains(&ix) {
            if let Some(pnode) = stack.pop() {
                if let Some(inode) = input.graph.node_weight(ix) {
                    if let Some(onode) = output.graph.node_weight_mut(ix) {
                        if !pnode.is_input() {
                            onode.clear_tile();
                            attrs.extend(inode.attrs().clone());
                            onode.set_target_loc(tile.loc().clone());
                            if ix == index {
                                instr.set_dst_id(&inode.id());
                                onode.set_cost(tile.cost());
                            } else {
                                onode.set_cost(0.0);
                            }
                        } else {
                            params.push(asm::Expr::new_ref(&inode.id(), inode.ty().clone()));
                            let childs = input.graph.neighbors_directed(ix, Direction::Outgoing);
                            for c in childs {
                                discard.insert(c);
                            }
                        }
                    }
                }
            }
        } else {
            let childs = input.graph.neighbors_directed(ix, Direction::Outgoing);
            for c in childs {
                discard.insert(c);
            }
        }
        if stack.is_empty() {
            break;
        }
    }
    for attr in attrs.iter() {
        instr.add_attr(attr.clone());
    }
    for param in params.iter() {
        instr.add_param(param.clone());
    }
    if let Some(onode) = output.graph.node_weight_mut(index) {
        let mut tile = tile.clone();
        tile.set_instr(instr);
        onode.set_tile(tile);
    }
    output
}

pub fn tree_selection(descriptor: &Descriptor, input: &Tree) -> Tree {
    let mut output = input.clone();
    let start = input.root_index().unwrap();
    let mut dfs = DfsPostOrder::new(&input.graph(), start);
    while let Some(ix) = dfs.next(&input.graph) {
        if let Some(node) = input.graph.node_weight(ix) {
            if !node.is_input() {
                for tile in descriptor.tiles.iter() {
                    let (is_match, cur_cost) = tree_match(tile, &output, ix);
                    if is_match {
                        let pat_cost = tile.cost();
                        if pat_cost < cur_cost {
                            output = tree_update(tile, &output, ix);
                        }
                    }
                }
            }
        }
    }
    output
}

pub fn tree_asmgen(input: &Tree) -> InstrMap {
    let mut map: InstrMap = InstrMap::new();
    let root_index = input.root_index().unwrap();
    let graph = input.graph.clone();
    let mut visit = DfsPostOrder::new(&graph, root_index);
    while let Some(ix) = visit.next(&graph) {
        if let Some(node) = graph.node_weight(ix) {
            if let Some(tile) = node.tile() {
                map.insert(tile.instr().dst_id(), tile.instr().clone());
            }
        }
    }
    map
}

pub fn tree_locgen(input: &Tree) -> LocMap {
    let mut map: LocMap = LocMap::new();
    let root_index = input.root_index().unwrap();
    let graph = input.graph.clone();
    let mut visit = DfsPostOrder::new(&graph, root_index);
    while let Some(ix) = visit.next(&graph) {
        if let Some(node) = graph.node_weight(ix) {
            if let Some(loc) = node.target_loc() {
                map.insert(node.id(), loc.clone());
            }
        }
    }
    map
}
