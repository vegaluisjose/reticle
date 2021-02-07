use crate::compiler::helpers::find_roots;
use crate::compiler::tree::*;
use crate::ir::ast::*;
use crate::tdl::ast::Pat;
use crate::util::errors::Error;
use std::collections::{HashMap, HashSet};
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
            fixed: false,
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
            fixed: false,
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

impl TryFrom<Def> for Forest {
    type Error = Error;
    fn try_from(def: Def) -> Result<Self, Self::Error> {
        let mut imap: HashMap<Id, Instr> = HashMap::new();
        for instr in def.body() {
            if let Some(term) = instr.dst().term() {
                if let Some(id) = term.id() {
                    imap.insert(id, instr.clone());
                }
            }
        }
        let mut forest = Forest::default();
        let input: Vec<ExprTerm> = def.input().clone().into();
        for e in input {
            if let Some(id) = e.id() {
                forest.add_visited(&id);
            }
        }
        let roots = find_roots(&def);
        for r in roots {
            let mut tree = Tree::default();
            let mut stack: Vec<(Id, u64)> = Vec::new();
            if let Some(instr) = imap.get(&r) {
                forest.add_visited(&r);
                let index = tree.add_node(instr)?;
                stack.push((r.clone(), index));
            }
            while let Some((curr, index)) = stack.pop() {
                if let Some(instr) = imap.get(&curr) {
                    let arg: Vec<ExprTerm> = instr.arg().clone().into();
                    for term in arg {
                        let id = term.get_id()?;
                        if imap.contains_key(&id) && !forest.was_visited(&id) {
                            if let Some(instr) = imap.get(&id) {
                                let to = tree.add_node(instr)?;
                                tree.add_edge(index, to);
                                forest.add_visited(&id);
                                stack.push((id, to));
                            }
                        } else {
                            let ty = term.get_ty()?;
                            let to = tree.add_input(&id, ty.clone());
                            tree.add_edge(index, to);
                        }
                    }
                }
            }
            forest.add_tree(tree);
        }
        Ok(forest)
    }
}

impl TryFrom<Pat> for Tree {
    type Error = Error;
    fn try_from(pat: Pat) -> Result<Self, Self::Error> {
        let mut imap: HashMap<Id, Instr> = HashMap::new();
        for tdl_instr in pat.body() {
            if let Some(term) = tdl_instr.dst().term() {
                if let Some(id) = term.id() {
                    let instr = Instr::from(tdl_instr.clone());
                    imap.insert(id, instr);
                }
            }
        }
        let mut visited: HashSet<Id> = HashSet::new();
        let input: Vec<ExprTerm> = pat.input().clone().into();
        for e in input {
            if let Some(id) = e.id() {
                visited.insert(id);
            }
        }
        let mut tree = Tree::default();
        let mut stack: Vec<(Id, u64)> = Vec::new();
        let root = pat.output().get_id(0)?;
        if let Some(instr) = imap.get(&root) {
            visited.insert(root.clone());
            let cost = pat.lat(); // change cost function for pattern here
            let index = tree.add_node_with_cost(instr, cost)?;
            stack.push((root.clone(), index));
        }
        while let Some((curr, index)) = stack.pop() {
            if let Some(instr) = imap.get(&curr) {
                let arg: Vec<ExprTerm> = instr.arg().clone().into();
                for term in arg {
                    let id = term.get_id()?;
                    if imap.contains_key(&id) && !visited.contains(&id) {
                        if let Some(instr) = imap.get(&id) {
                            let to = tree.add_node(instr)?;
                            tree.add_edge(index, to);
                            visited.insert(id.clone());
                            stack.push((id, to));
                        }
                    } else {
                        let ty = term.get_ty()?;
                        let to = tree.add_input(&id, ty.clone());
                        tree.add_edge(index, to);
                    }
                }
            }
        }
        Ok(tree)
    }
}
