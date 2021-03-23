use crate::asm::ast as asm;
use crate::ir::ast as ir;
use crate::tdl::ast::Pat;
use crate::util::errors::Error;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum NodeOp {
    Wire(ir::OpWire),
    Comp(ir::OpComp),
    Inp,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub index: u64,
    pub id: ir::Id,
    pub ty: ir::Ty,
    pub op: NodeOp,
    pub attr: ir::Expr,
    pub prim: ir::Prim,
    pub cost: u64,
    pub staged: bool,
    pub committed: bool,
    pub pat: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Tree {
    pub index: u64,
    pub node: HashMap<u64, Node>,
    pub edge: HashMap<u64, Vec<u64>>,
}

pub fn tree_roots_from_def(def: &ir::Def) -> Vec<ir::Id> {
    let mut count: HashMap<ir::Id, u64> = HashMap::new();
    // store compute instructions
    for instr in def.body() {
        if instr.is_comp() {
            if let Some(term) = instr.dst().term() {
                if let Some(id) = term.id() {
                    count.insert(id, 0);
                }
            }
        }
    }
    // calculate the number of times compute instructions are used
    for instr in def.body() {
        let arg: Vec<ir::ExprTerm> = instr.arg().clone().into();
        for e in arg {
            if let Some(id) = e.id() {
                if let Some(val) = count.get_mut(&id) {
                    *val += 1;
                }
            }
        }
    }
    let mut root: Vec<ir::Id> = Vec::new();
    // a node is a root if it is used more than once
    for (k, v) in count {
        if v > 1 {
            root.push(k);
        }
    }
    // add outputs as roots
    let output: Vec<ir::ExprTerm> = def.output().clone().into();
    for e in output {
        if let Some(id) = e.id() {
            root.push(id);
        }
    }
    root
}

pub fn tree_from_pats(pmap: &HashMap<String, Pat>) -> Result<HashMap<String, Tree>, Error> {
    let mut pattern: HashMap<String, Tree> = HashMap::new();
    for (n, p) in pmap {
        pattern.insert(n.clone(), Tree::try_from(p.clone())?);
    }
    Ok(pattern)
}

pub fn tree_from_prog(prog: &ir::Prog) -> Result<Vec<Tree>, Error> {
    if let Some(main) = prog.get("main") {
        Ok(main.clone().try_into()?)
    } else {
        Err(Error::new_compiler_error("converting to tree"))
    }
}

pub fn imap_from_prog(prog: &ir::Prog) -> Result<ir::InstrMap, Error> {
    if let Some(main) = prog.get("main") {
        Ok(ir::InstrMap::from(main.clone()))
    } else {
        Err(Error::new_compiler_error("converting to imap"))
    }
}

pub fn is_valid_change(block: &Tree, pat: &Tree, start: u64) -> (bool, u64) {
    let mut pstack = pat.bfs(0);
    pstack.reverse();
    let mut bstack: VecDeque<u64> = VecDeque::new();
    bstack.push_back(start);
    let mut is_match = true;
    let mut bcost: u64 = 0;
    if let Some(proot) = pat.node(0) {
        let pcost = proot.cost();
        while let Some(bindex) = bstack.pop_front() {
            if let Some(pindex) = pstack.pop() {
                if let Some(bnode) = block.node(bindex) {
                    if let Some(pnode) = pat.node(pindex) {
                        if pnode.ty() != bnode.ty()
                            || (!pnode.is_inp() && pnode.op() != bnode.op())
                            || (!pnode.is_inp()
                                && !bnode.prim().is_any()
                                && pnode.prim() != bnode.prim())
                            || (!pnode.is_inp() && pnode.attr() != bnode.attr())
                            || (!pnode.is_inp() && bnode.is_committed())
                            || (pnode.is_inp() && !bnode.is_free())
                        {
                            is_match = false;
                        }
                        if !pnode.is_inp() && bnode.cost() == u64::MAX {
                            bcost = bnode.cost();
                        } else if !pnode.is_inp() && bnode.cost() != u64::MAX {
                            bcost += bnode.cost();
                        }
                        if is_match && !pnode.is_inp() {
                            if let Some(edge) = block.edge(bindex) {
                                for e in edge {
                                    bstack.push_back(*e);
                                }
                            }
                        }
                    }
                }
            }
        }
        (is_match & (pcost < bcost), pcost)
    } else {
        (false, u64::MAX)
    }
}

pub fn tree_update(block: &Tree, pat: &Tree, target: u64, pat_name: &str, pat_cost: u64) -> Tree {
    let mut btree = block.clone();
    let mut pstack = pat.bfs(0);
    pstack.reverse();
    let mut bstack: VecDeque<u64> = VecDeque::new();
    bstack.push_back(target);
    while let Some(bindex) = bstack.pop_front() {
        if let Some(pindex) = pstack.pop() {
            if let Some(pnode) = pat.node(pindex) {
                if !pnode.is_inp() {
                    if let Some(bnode) = btree.node_mut(bindex) {
                        bnode.clear_pat();
                        bnode.set_cost(0);
                        bnode.stage();
                    }
                    if let Some(edge) = block.edge(bindex) {
                        for e in edge {
                            bstack.push_back(*e);
                        }
                    }
                }
            }
        }
    }
    if let Some(bnode) = btree.node_mut(target) {
        bnode.set_pat(pat_name);
        bnode.set_cost(pat_cost);
    }
    btree
}

pub fn input_map(block: &Tree, pat: &Tree, target: u64) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    let mut pstack = pat.bfs(0);
    pstack.reverse();
    let mut bstack: VecDeque<u64> = VecDeque::new();
    bstack.push_back(target);
    while let Some(bindex) = bstack.pop_front() {
        if let Some(pindex) = pstack.pop() {
            if let Some(pnode) = pat.node(pindex) {
                if pnode.is_inp() {
                    if let Some(bnode) = block.node(bindex) {
                        map.insert(pnode.id(), bnode.id());
                    }
                } else if let Some(edge) = block.edge(bindex) {
                    for e in edge {
                        bstack.push_back(*e);
                    }
                }
            }
        }
    }
    map
}

pub fn output_map(block: &Tree, pat: &Tree, target: u64) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    if let Some(pnode) = pat.node(0) {
        if let Some(bnode) = block.node(target) {
            map.insert(pnode.id(), bnode.id());
        }
    }
    map
}

pub fn rename_expr(map: &HashMap<String, String>, input: &asm::Expr) -> Result<asm::Expr, Error> {
    let iterm: Vec<asm::ExprTerm> = input.clone().into();
    let mut oterm: Vec<asm::ExprTerm> = Vec::new();
    for e in iterm {
        if let Some(id) = map.get(&e.get_id()?) {
            let ty = e.get_ty()?;
            oterm.push(asm::ExprTerm::Var(id.clone(), ty.clone()));
        }
    }
    if oterm.len() == 1 {
        Ok(asm::Expr::from(oterm[0].clone()))
    } else {
        let tup: asm::ExprTup = oterm.into();
        Ok(asm::Expr::from(tup))
    }
}

pub fn tree_select(blocks: &[Tree], pmap: &HashMap<String, Tree>) -> Result<Vec<Tree>, Error> {
    let mut res: Vec<Tree> = Vec::new();
    for btree in blocks {
        let mut ctree = btree.clone();
        let cuts = btree.cut(0);
        for cut in cuts {
            for (pname, ptree) in pmap {
                let (is_valid, cost) = is_valid_change(&ctree, &ptree, cut);
                if is_valid {
                    ctree = tree_update(&ctree, &ptree, cut, &pname, cost);
                }
            }
        }
        ctree.commit();
        res.push(ctree);
    }
    Ok(res)
}

pub fn tree_codegen(
    iset: &mut HashSet<ir::Id>,
    imap: &ir::InstrMap,
    block: &Tree,
    tmap: &HashMap<String, Tree>,
    pmap: &HashMap<String, Pat>,
) -> Result<Vec<asm::Instr>, Error> {
    let mut body: Vec<asm::Instr> = Vec::new();
    let mut indices = block.bfs(0);
    // bottom-up code generation
    indices.reverse();
    for index in indices {
        if let Some(node) = block.node(index) {
            if node.is_committed() {
                if let Some(name) = node.pat() {
                    if let Some(tree) = tmap.get(name) {
                        if let Some(pat) = pmap.get(name) {
                            let input = input_map(block, tree, index);
                            let output = output_map(block, tree, index);
                            let dst = rename_expr(&output, pat.output())?;
                            let arg = rename_expr(&input, pat.input())?;
                            let op = asm::OpAsm::from(name.clone());
                            let loc = asm::Loc {
                                prim: pat.prim().clone(),
                                x: asm::ExprCoord::Any,
                                y: asm::ExprCoord::Any,
                            };
                            let asm = asm::InstrAsm { op, dst, arg, loc };
                            body.push(asm::Instr::from(asm));
                        }
                    }
                }
            } else if !node.is_staged() && node.is_wire() {
                if let Some(instr) = imap.get(&node.id()) {
                    if !iset.contains(&node.id()) {
                        let wire = asm::InstrWire::try_from(instr.clone())?;
                        body.push(asm::Instr::from(wire));
                        iset.insert(node.id());
                    }
                }
            } // TODO: add error for uncovered node
        }
    }
    Ok(body)
}
