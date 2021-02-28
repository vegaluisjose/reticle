use crate::asm::ast as asm;
use crate::compiler::tree::*;
use crate::ir::ast as ir;
use crate::tdl::ast as tdl;
use crate::tdl::parser::TDLParser;
use crate::util::errors::Error;
use crate::util::file::create_abs_path;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

impl TryFrom<ir::InstrWire> for Node {
    type Error = Error;
    fn try_from(input: ir::InstrWire) -> Result<Self, Self::Error> {
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
            prim: ir::Prim::Any,
            cost: 0,
            staged: false,
            committed: false,
            pat: None,
        })
    }
}

impl TryFrom<ir::InstrComp> for Node {
    type Error = Error;
    fn try_from(input: ir::InstrComp) -> Result<Self, Self::Error> {
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

impl TryFrom<ir::Instr> for Node {
    type Error = Error;
    fn try_from(input: ir::Instr) -> Result<Self, Self::Error> {
        match input {
            ir::Instr::Wire(instr) => Ok(Node::try_from(instr)?),
            ir::Instr::Comp(instr) => Ok(Node::try_from(instr)?),
            _ => Err(Error::new_conv_error(
                "call node conversion is not supported",
            )),
        }
    }
}

fn build_tree_mut(
    map: &ir::InstrMap,
    visited: &mut HashSet<ir::Id>,
    input: &ir::Expr,
    root: &str,
    cost: u64,
) -> Result<Tree, Error> {
    let input_map: ir::TermMap = input.clone().into();
    for id in input_map.keys() {
        visited.insert(id.clone());
    }
    let mut tree = Tree::default();
    let mut stack: Vec<(ir::Id, u64)> = Vec::new();
    if let Some(instr) = map.get(root) {
        visited.insert(root.to_string());
        let index = tree.add_node_with_cost(instr, cost)?;
        stack.push((root.to_string(), index));
    }
    while let Some((curr, index)) = stack.pop() {
        if let Some(instr) = map.get(&curr) {
            let arg: Vec<ir::ExprTerm> = instr.arg().clone().into();
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

impl TryFrom<ir::Def> for Vec<Tree> {
    type Error = Error;
    fn try_from(def: ir::Def) -> Result<Self, Self::Error> {
        let map = ir::InstrMap::from(def.clone());
        let mut res: Vec<Tree> = Vec::new();
        let roots = tree_roots_from_def(&def);
        let mut visited: HashSet<ir::Id> = HashSet::new();
        for r in roots {
            let tree = build_tree_mut(&map, &mut visited, def.input(), &r, u64::MAX)?;
            res.push(tree);
        }
        Ok(res)
    }
}

impl TryFrom<tdl::Pat> for Tree {
    type Error = Error;
    fn try_from(pat: tdl::Pat) -> Result<Self, Self::Error> {
        let map = ir::InstrMap::from(pat.clone());
        let mut visited: HashSet<ir::Id> = HashSet::new();
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

fn select(prog: &ir::Prog) -> Result<asm::Prog, Error> {
    let lut_tdl = create_abs_path("examples/target/ultrascale/lut.tdl");
    let dsp_tdl = create_abs_path("examples/target/ultrascale/dsp.tdl");
    let lut = TDLParser::parse_from_file(lut_tdl)?;
    let dsp = TDLParser::parse_from_file(dsp_tdl)?;
    let lmap = tree_from_pats(lut.pat())?;
    let dmap = tree_from_pats(dsp.pat())?;
    let imap = imap_from_prog(&prog)?;
    let blks = tree_from_prog(&prog)?;
    let blks = tree_select(&blks, &dmap)?;
    let blks = tree_select(&blks, &lmap)?;
    let mut body: Vec<asm::Instr> = Vec::new();
    let mut iset: HashSet<ir::Id> = HashSet::new();
    let tree_map: HashMap<String, Tree> = lmap.into_iter().chain(dmap).collect();
    let pat_map: HashMap<String, tdl::Pat> = lut
        .pat()
        .clone()
        .into_iter()
        .chain(dsp.pat().clone())
        .collect();
    for blk in blks {
        body.extend(tree_codegen(&mut iset, &imap, &blk, &tree_map, &pat_map)?);
    }
    let mut res = asm::Prog::default();
    if let Some(main) = prog.get("main") {
        res.set_sig(main.sig().clone());
        res.set_body(body);
        Ok(res)
    } else {
        Err(Error::new_compiler_error(
            "Prog must have a main definition",
        ))
    }
}

impl TryFrom<ir::Prog> for asm::Prog {
    type Error = Error;
    fn try_from(prog: ir::Prog) -> Result<Self, Self::Error> {
        Ok(select(&prog)?)
    }
}
