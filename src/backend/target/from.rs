use crate::backend::asm::ast::*;
use crate::backend::target::descriptor::*;
use crate::backend::target::spec::*;
use crate::lang::ast::PrimOp;
use crate::passes::map::partition::tree::{Tree, TreeNode, TreeOp};
use std::str::FromStr;

impl From<SpecInstr> for Instr {
    fn from(spec_instr: SpecInstr) -> Self {
        Instr::Asm {
            id: String::new(),
            ty: Ty::from_str(&spec_instr.ty()).unwrap(),
            op: spec_instr.name(),
            attrs: Vec::new(),
            params: Vec::new(),
            loc: Loc::new_with_hole(LocTy::from_str(&spec_instr.loc()).unwrap()),
        }
    }
}

impl From<SpecInstr> for Tree {
    fn from(spec_instr: SpecInstr) -> Self {
        let mut cnt: u32 = 0;
        let mut tree = Tree::new(&cnt.to_string());
        let mut stack_node: Vec<SpecExpr> = Vec::new();
        let mut stack_id: Vec<u32> = Vec::new();
        stack_node.push(spec_instr.expr.clone());
        stack_id.push(cnt);
        // create nodes
        while !stack_node.is_empty() && !stack_id.is_empty() {
            let expr = stack_node.pop().unwrap();
            match expr {
                SpecExpr::Input(ty) => {
                    let name = cnt.to_string();
                    let ty = Ty::from_str(&ty).unwrap();
                    let node = TreeNode::new_input(&name, ty);
                    let src_id = stack_id.pop().unwrap().to_string();
                    let dst_id = cnt.to_string();
                    tree.add_node(&name, node);
                    tree.add_edge(&src_id, &dst_id);
                    cnt += 1;
                }
                SpecExpr::UnOp(op, input) => {
                    let name = cnt.to_string();
                    let ty = Ty::from_str(&spec_instr.ty()).unwrap();
                    let primop = PrimOp::from_str(&op).unwrap();
                    let op = TreeOp::from(primop);
                    let node = TreeNode::new(&name, ty, op);
                    tree.add_node(&name, node);
                    // if it is not root
                    if cnt > 0 {
                        let src_id = stack_id.pop().unwrap().to_string();
                        let dst_id = cnt.to_string();
                        tree.add_edge(&src_id, &dst_id);
                    } else {
                        stack_id.pop();
                    }
                    stack_id.push(cnt);
                    cnt += 1;
                    stack_node.push(input.as_ref().clone());
                }
                SpecExpr::BinOp(op, lhs, rhs) => {
                    let name = cnt.to_string();
                    let ty = Ty::from_str(&spec_instr.ty()).unwrap();
                    let primop = PrimOp::from_str(&op).unwrap();
                    let op = TreeOp::from(primop);
                    let node = TreeNode::new(&name, ty, op);
                    tree.add_node(&name, node);
                    // if it is not root
                    if cnt > 0 {
                        let src_id = stack_id.pop().unwrap().to_string();
                        let dst_id = cnt.to_string();
                        tree.add_edge(&src_id, &dst_id);
                    } else {
                        stack_id.pop();
                    }
                    stack_id.push(cnt);
                    stack_id.push(cnt);
                    cnt += 1;
                    stack_node.push(rhs.as_ref().clone());
                    stack_node.push(lhs.as_ref().clone());
                }
                SpecExpr::TerOp(op, con, tru, fal) => {
                    let name = cnt.to_string();
                    let ty = Ty::from_str(&spec_instr.ty()).unwrap();
                    let primop = PrimOp::from_str(&op).unwrap();
                    let op = TreeOp::from(primop);
                    let node = TreeNode::new(&name, ty, op);
                    tree.add_node(&name, node);
                    // if it is not root
                    if cnt > 0 {
                        let src_id = stack_id.pop().unwrap().to_string();
                        let dst_id = cnt.to_string();
                        tree.add_edge(&src_id, &dst_id);
                    } else {
                        stack_id.pop();
                    }
                    stack_id.push(cnt);
                    stack_id.push(cnt);
                    stack_id.push(cnt);
                    cnt += 1;
                    stack_node.push(fal.as_ref().clone());
                    stack_node.push(tru.as_ref().clone());
                    stack_node.push(con.as_ref().clone());
                }
            }
        }
        tree
    }
}

impl From<SpecInstr> for Tile {
    fn from(spec_instr: SpecInstr) -> Self {
        Tile {
            instr: Instr::from(spec_instr.clone()),
            pattern: Tree::from(spec_instr),
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
