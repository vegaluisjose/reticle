use crate::errors::Error;
use crate::tree::helpers::tree_try_from_map;
use crate::tree::*;
use pat::ast as pat;
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

impl TryFrom<InstrPrim> for Node {
    type Error = Error;
    fn try_from(input: InstrPrim) -> Result<Self, Self::Error> {
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
            Instr::Prim(instr) => Ok(Node::try_from(instr)?),
            _ => Err(Error::new_tree_error(
                "call node conversion is not supported",
            )),
        }
    }
}

impl TryFrom<pat::Pat> for Tree {
    type Error = Error;
    fn try_from(pat: pat::Pat) -> Result<Self, Self::Error> {
        let map = InstrMap::from(pat.clone());
        let mut visited: HashSet<Id> = HashSet::new();
        let tree = tree_try_from_map(&map, &mut visited, pat.input(), &pat.output().get_id(0)?, 0)?;
        Ok(tree)
    }
}
