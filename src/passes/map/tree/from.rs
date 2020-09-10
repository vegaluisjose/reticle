use crate::lang::ast::{Instr, PrimOp};
use crate::passes::map::dfg::{Dfg, DfgIx};
use crate::passes::map::tree::partition::Partition;
use crate::passes::map::tree::*;

impl From<PrimOp> for TreeOp {
    fn from(primop: PrimOp) -> Self {
        TreeOp::Prim(primop)
    }
}

impl From<Instr> for TreeNode {
    fn from(instr: Instr) -> Self {
        let mut node = TreeNode::new(
            &instr.dst_id(),
            instr.dst_ty().clone(),
            TreeOp::from(instr.prim_op().clone()),
        );
        for attr in instr.attrs().iter() {
            node.add_attr(attr.clone());
        }
        node
    }
}

impl From<Dfg> for Partition {
    fn from(dfg: Dfg) -> Self {
        let mut dfg = dfg;
        let mut partition = Partition::new();
        for (id, root) in dfg.roots().clone().iter() {
            let mut tree = Tree::new();
            let mut stack: Vec<DfgIx> = Vec::new();
            stack.push(*root);
            while !stack.is_empty() {
                if let Some(src_ix) = stack.pop() {
                    if let Some(src_node) = dfg.graph.node_weight_mut(src_ix) {
                        src_node.set_visited();
                    }
                    if let Some(src_node) = dfg.graph.node_weight(src_ix) {
                        if !tree.contains_node_with_id(&src_node.id()) {
                            tree.add_node(&src_node.id(), TreeNode::from(src_node.instr().clone()));
                        }
                        let incoming = dfg.get_incoming_nodes(src_ix);
                        for param in src_node.instr().params().iter() {
                            if let Some(dst_ix) = incoming.get(&param.id()) {
                                if let Some(dst_node) = dfg.graph.node_weight(*dst_ix) {
                                    tree.add_node(
                                        &dst_node.id(),
                                        TreeNode::from(dst_node.instr().clone()),
                                    );
                                    tree.add_edge(&src_node.id(), &dst_node.id());
                                }
                            } else {
                                tree.add_node(
                                    &param.id(),
                                    TreeNode::new_input(&param.id(), param.ty().clone()),
                                );
                                tree.add_edge(&src_node.id(), &param.id());
                            }
                        }
                        let incoming_ix: Vec<DfgIx> = incoming.values().cloned().collect();
                        stack.extend(incoming_ix);
                    }
                }
            }
            partition.insert(id.to_string(), tree);
        }
        partition
    }
}
