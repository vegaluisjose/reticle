use crate::compiler::tree::*;
use crate::compiler::utils::tree_roots_from_def;
use crate::ir::ast::*;
use crate::tdl::ast::Pat;
use crate::util::errors::Error;
use std::collections::HashSet;
use std::convert::TryFrom;

impl TryFrom<InstrWire> for Node {
    type Error = Error;
    fn try_from(input: InstrWire) -> Result<Self, Self::Error> {
        let id = input.dst().get_id(0)?;
        let ty = input.dst().get_ty(0)?;
        let op = NodeOp::from(input.op().clone());
        let attr = input.attr().clone();
        Ok(Node {
            index: 0,
            id,
            ty: ty.clone(),
            op,
            attr,
            prim: Prim::Any,
            cost: 0,
            staged: false,
            committed: false,
            pat: None,
        })
    }
}

impl TryFrom<InstrComp> for Node {
    type Error = Error;
    fn try_from(input: InstrComp) -> Result<Self, Self::Error> {
        let id = input.dst().get_id(0)?;
        let ty = input.dst().get_ty(0)?;
        let op = NodeOp::from(input.op().clone());
        let attr = input.attr().clone();
        let prim = input.prim().clone();
        Ok(Node {
            index: 0,
            id,
            ty: ty.clone(),
            op,
            attr,
            prim,
            cost: u64::MAX,
            staged: false,
            committed: false,
            pat: None,
        })
    }
}

impl TryFrom<Instr> for Node {
    type Error = Error;
    fn try_from(input: Instr) -> Result<Self, Self::Error> {
        match input {
            Instr::Wire(instr) => Ok(Node::try_from(instr)?),
            Instr::Comp(instr) => Ok(Node::try_from(instr)?),
            _ => Err(Error::new_conv_error(
                "call node conversion is not supported",
            )),
        }
    }
}

pub fn build_tree_mut(
    map: &InstrMap,
    visited: &mut HashSet<Id>,
    input: &Expr,
    root: &str,
    cost: u64,
) -> Result<Tree, Error> {
    let input_map: TermMap = input.clone().into();
    for id in input_map.keys() {
        visited.insert(id.clone());
    }
    let mut tree = Tree::default();
    let mut stack: Vec<(Id, u64)> = Vec::new();
    if let Some(instr) = map.get(root) {
        visited.insert(root.to_string());
        let index = tree.add_node_with_cost(instr, cost)?;
        stack.push((root.to_string(), index));
    }
    while let Some((curr, index)) = stack.pop() {
        if let Some(instr) = map.get(&curr) {
            let arg: Vec<ExprTerm> = instr.arg().clone().into();
            for term in arg {
                let id = term.get_id()?;
                if let Some(instr) = map.get(&id) {
                    // add node if was not visited
                    if map.contains_key(&id) && !visited.contains(&id) {
                        let to = tree.add_node(instr)?;
                        tree.add_edge(index, to);
                        visited.insert(id.clone());
                        stack.push((id.clone(), to));
                    // if visited and it is wire, then duplicate
                    } else if instr.is_wire() {
                        let to = tree.add_node(instr)?;
                        tree.add_edge(index, to);
                    // else make it an input
                    } else {
                        let ty = term.get_ty()?;
                        let to = tree.add_input(&id, ty.clone());
                        tree.add_edge(index, to);
                    }
                } else if let Some(term) = input_map.get(&id) {
                    let ty = term.get_ty()?;
                    let to = tree.add_input(&id, ty.clone());
                    tree.add_edge(index, to);
                }
            }
        }
    }
    Ok(tree)
}

impl TryFrom<Def> for Vec<Tree> {
    type Error = Error;
    fn try_from(def: Def) -> Result<Self, Self::Error> {
        let map = InstrMap::from(def.clone());
        let mut res: Vec<Tree> = Vec::new();
        let roots = tree_roots_from_def(&def);
        let mut visited: HashSet<Id> = HashSet::new();
        for r in roots {
            let tree = build_tree_mut(&map, &mut visited, def.input(), &r, u64::MAX)?;
            res.push(tree);
        }
        Ok(res)
    }
}

impl TryFrom<Pat> for Tree {
    type Error = Error;
    fn try_from(pat: Pat) -> Result<Self, Self::Error> {
        let map = InstrMap::from(pat.clone());
        let mut visited: HashSet<Id> = HashSet::new();
        let tree = build_tree_mut(
            &map,
            &mut visited,
            pat.input(),
            &pat.output().get_id(0)?,
            pat.lat(),
        )?;
        Ok(tree)
    }
}
