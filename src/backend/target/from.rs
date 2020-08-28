use crate::backend::asm::ast::{InstrPrim, Loc, LocTy, Ty};
use crate::backend::target::descriptor::*;
use crate::backend::target::spec::*;
use crate::passes::map::tree::{Tree, TreeNode, TreeOp, TreeTy};
use std::str::FromStr;

impl From<SpecInstr> for InstrPrim {
    fn from(spec_instr: SpecInstr) -> Self {
        let ty = Ty::from_str(&spec_instr.ty()).unwrap();
        let loc = Loc::new_with_hole(LocTy::from_str(&spec_instr.loc()).unwrap());
        let mut instr = InstrPrim::new(loc);
        instr.set_op(&spec_instr.name());
        instr.set_dst_ty(ty);
        instr
    }
}

impl From<SpecInstr> for Tree {
    fn from(spec_instr: SpecInstr) -> Self {
        let mut cnt: u32 = 0;
        let mut tree = Tree::new();
        let mut stack_node: Vec<SpecExpr> = Vec::new();
        let mut stack_id: Vec<u32> = Vec::new();
        stack_node.push(spec_instr.expr.clone());
        stack_id.push(cnt);
        while !stack_node.is_empty() && !stack_id.is_empty() {
            let expr = stack_node.pop().unwrap();
            let cost = spec_instr.delay() as f32;
            match expr {
                SpecExpr::Input(ty) => {
                    let name = cnt.to_string();
                    let ty = TreeTy::from_str(&ty).unwrap();
                    let node = TreeNode::new_input(&name, ty);
                    tree.add_node(&name, node);
                    let src_id = stack_id.pop().unwrap().to_string();
                    let dst_id = cnt.to_string();
                    tree.add_edge(&src_id, &dst_id);
                }
                SpecExpr::UnOp(op, input) => {
                    let name = cnt.to_string();
                    let ty = TreeTy::from_str(&spec_instr.ty()).unwrap();
                    let op = TreeOp::from_str(&op).unwrap();
                    let node = if cnt == 0 {
                        // root
                        TreeNode::new_with_cost(&name, ty, op, cost)
                    } else {
                        TreeNode::new_with_cost(&name, ty, op, 0.0)
                    };
                    tree.add_node(&name, node);
                    if cnt == 0 {
                        // root
                        stack_id.pop();
                    } else {
                        let src_id = stack_id.pop().unwrap().to_string();
                        let dst_id = cnt.to_string();
                        tree.add_edge(&src_id, &dst_id);
                    }
                    stack_id.push(cnt);
                    stack_node.push(input.as_ref().clone());
                }
                SpecExpr::BinOp(op, lhs, rhs) => {
                    let name = cnt.to_string();
                    let ty = TreeTy::from_str(&spec_instr.ty()).unwrap();
                    let op = TreeOp::from_str(&op).unwrap();
                    let node = if cnt == 0 {
                        // root
                        TreeNode::new_with_cost(&name, ty, op, cost)
                    } else {
                        TreeNode::new_with_cost(&name, ty, op, 0.0)
                    };
                    tree.add_node(&name, node);
                    if cnt == 0 {
                        // root
                        stack_id.pop();
                    } else {
                        let src_id = stack_id.pop().unwrap().to_string();
                        let dst_id = cnt.to_string();
                        tree.add_edge(&src_id, &dst_id);
                    }
                    stack_id.push(cnt);
                    stack_id.push(cnt);
                    stack_node.push(rhs.as_ref().clone());
                    stack_node.push(lhs.as_ref().clone());
                }
                SpecExpr::TerOp(op, con, tru, fal) => {
                    let name = cnt.to_string();
                    let ty = TreeTy::from_str(&spec_instr.ty()).unwrap();
                    let op = TreeOp::from_str(&op).unwrap();
                    let node = if cnt == 0 {
                        // root
                        TreeNode::new_with_cost(&name, ty, op, cost)
                    } else {
                        TreeNode::new_with_cost(&name, ty, op, 0.0)
                    };
                    tree.add_node(&name, node);
                    if cnt == 0 {
                        // root
                        stack_id.pop();
                    } else {
                        let src_id = stack_id.pop().unwrap().to_string();
                        let dst_id = cnt.to_string();
                        tree.add_edge(&src_id, &dst_id);
                    }
                    stack_id.push(cnt);
                    stack_id.push(cnt);
                    stack_id.push(cnt);
                    stack_node.push(fal.as_ref().clone());
                    stack_node.push(tru.as_ref().clone());
                    stack_node.push(con.as_ref().clone());
                }
            }
            cnt += 1;
        }
        tree
    }
}

impl From<SpecInstr> for Tile {
    fn from(spec_instr: SpecInstr) -> Self {
        Tile {
            instr: InstrPrim::from(spec_instr.clone()),
            pattern: Tree::from(spec_instr.clone()),
            loc: LocTy::from_str(&spec_instr.loc()).unwrap(),
        }
    }
}

impl From<Spec> for Descriptor {
    fn from(spec: Spec) -> Self {
        let mut tiles: Vec<Tile> = Vec::new();
        for instr in spec.isa.iter() {
            tiles.push(Tile::from(instr.clone()));
        }
        Descriptor {
            tiles: tiles.to_vec(),
        }
    }
}
