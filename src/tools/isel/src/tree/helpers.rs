use crate::errors::Error;
use crate::tree::*;
use pat::ast as pat;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::path::Path;
use xim::ast as xim;

impl Node {
    pub fn index(&self) -> u64 {
        self.index
    }
    pub fn id(&self) -> Id {
        self.id.to_string()
    }
    pub fn ty(&self) -> &Ty {
        &self.ty
    }
    pub fn op(&self) -> &NodeOp {
        &self.op
    }
    pub fn is_inp(&self) -> bool {
        matches!(self.op, NodeOp::Inp)
    }
    pub fn is_wire(&self) -> bool {
        matches!(self.op, NodeOp::Wire(_))
    }
    pub fn is_prim(&self) -> bool {
        matches!(self.op, NodeOp::Prim(_))
    }
    pub fn attr(&self) -> &Expr {
        &self.attr
    }
    pub fn prim(&self) -> &Prim {
        &self.prim
    }
    pub fn cost(&self) -> u64 {
        self.cost
    }
    pub fn pat(&self) -> Option<&String> {
        self.pat.as_ref()
    }
    pub fn is_staged(&self) -> bool {
        self.staged
    }
    pub fn is_committed(&self) -> bool {
        self.committed
    }
    pub fn is_free(&self) -> bool {
        !(self.staged | self.committed)
    }
    pub fn set_index(&mut self, index: u64) {
        self.index = index;
    }
    pub fn set_cost(&mut self, cost: u64) {
        self.cost = cost;
    }
    pub fn stage(&mut self) {
        self.staged = true;
    }
    pub fn commit(&mut self) {
        self.committed = true;
    }
    pub fn set_pat(&mut self, name: &str) {
        self.pat = Some(name.to_string());
    }
    pub fn clear_staged(&mut self) {
        self.staged = false;
    }
    pub fn clear_commit(&mut self) {
        self.committed = false;
    }
    pub fn clear_pat(&mut self) {
        self.pat = None;
    }
}

impl Tree {
    pub fn index(&self) -> u64 {
        self.index
    }
    pub fn node_map(&self) -> &HashMap<u64, Node> {
        &self.node
    }
    pub fn edge_map(&self) -> &HashMap<u64, Vec<u64>> {
        &self.edge
    }
    pub fn node(&self, index: u64) -> Option<&Node> {
        self.node.get(&index)
    }
    pub fn edge(&self, index: u64) -> Option<&Vec<u64>> {
        self.edge.get(&index)
    }
    pub fn dfg(&self, start: u64) -> Vec<u64> {
        let mut res: Vec<u64> = Vec::new();
        let mut stack: Vec<u64> = Vec::new();
        stack.push(start);
        while let Some(cur) = stack.pop() {
            res.push(cur);
            if let Some(edge) = self.edge(cur) {
                for e in edge {
                    stack.push(*e);
                }
            }
        }
        res
    }
    pub fn bfs(&self, start: u64) -> Vec<u64> {
        let mut res: Vec<u64> = Vec::new();
        let mut stack: VecDeque<u64> = VecDeque::new();
        stack.push_back(start);
        while let Some(cur) = stack.pop_front() {
            res.push(cur);
            if let Some(edge) = self.edge(cur) {
                for e in edge {
                    stack.push_back(*e);
                }
            }
        }
        res
    }
    pub fn bfs_bound(&self, start: u64, len: usize) -> Vec<u64> {
        let mut res: Vec<u64> = Vec::new();
        let mut stack: VecDeque<u64> = VecDeque::new();
        stack.push_back(start);
        while let Some(cur) = stack.pop_front() {
            res.push(cur);
            if let Some(edge) = self.edge(cur) {
                for e in edge {
                    if stack.len() < len {
                        stack.push_back(*e);
                    }
                }
            }
        }
        res
    }
    pub fn cut(&self, start: u64) -> Vec<u64> {
        let mut res: Vec<u64> = Vec::new();
        let mut stack: Vec<u64> = Vec::new();
        stack.push(start);
        while let Some(cur) = stack.pop() {
            if let Some(node) = self.node.get(&cur) {
                if node.is_prim() && !node.is_committed() {
                    res.push(cur);
                }
            }
            if let Some(edge) = self.edge(cur) {
                for e in edge {
                    stack.push(*e);
                }
            }
        }
        res.reverse();
        res
    }
    pub fn node_mut(&mut self, index: u64) -> Option<&mut Node> {
        self.node.get_mut(&index)
    }
    pub fn new_index(&mut self) -> u64 {
        let curr = self.index;
        self.index += 1;
        curr
    }
    pub fn add_node(&mut self, instr: &Instr) -> Result<u64, Error> {
        let curr = self.new_index();
        let mut node = Node::try_from(instr.clone())?;
        node.set_index(curr);
        self.node.insert(curr, node);
        self.edge.insert(curr, vec![]);
        Ok(curr)
    }
    pub fn add_node_with_cost(&mut self, instr: &Instr, cost: u64) -> Result<u64, Error> {
        let curr = self.new_index();
        let mut node = Node::try_from(instr.clone())?;
        node.set_index(curr);
        node.set_cost(cost);
        self.node.insert(curr, node);
        self.edge.insert(curr, vec![]);
        Ok(curr)
    }
    pub fn add_input(&mut self, id: &str, ty: Ty) -> u64 {
        let op = NodeOp::Inp;
        let curr = self.new_index();
        let mut node = Node {
            index: curr,
            id: id.to_string(),
            ty,
            op,
            attr: Expr::default(),
            prim: Prim::Any,
            cost: 0,
            staged: false,
            committed: false,
            pat: None,
        };
        node.set_index(curr);
        self.node.insert(curr, node);
        curr
    }
    pub fn add_edge(&mut self, from: u64, to: u64) {
        if let Some(edges) = self.edge.get_mut(&from) {
            edges.push(to);
        }
    }
    pub fn commit(&mut self) {
        let index = self.bfs(0);
        for i in index {
            if let Some(node) = self.node_mut(i) {
                if !node.is_inp() && node.pat().is_some() && node.is_staged() {
                    node.commit();
                }
            }
        }
    }
}

pub fn tree_roots_from_def(def: &Def) -> Vec<Id> {
    let mut count: HashMap<Id, u64> = HashMap::new();
    // store compute instructions
    for instr in def.body() {
        if instr.is_prim() {
            if let Some(term) = instr.dst().term() {
                if let Some(id) = term.id() {
                    count.insert(id, 0);
                }
            }
        }
    }
    // calculate the number of times compute instructions are used
    for instr in def.body() {
        let arg: Vec<ExprTerm> = instr.arg().clone().into();
        for e in arg {
            if let Some(id) = e.id() {
                if let Some(val) = count.get_mut(&id) {
                    *val += 1;
                }
            }
        }
    }
    let mut root: Vec<Id> = Vec::new();
    // a node is a root if it is used more than once
    for (k, v) in count {
        if v > 1 {
            root.push(k);
        }
    }
    // add outputs as roots
    let output: Vec<ExprTerm> = def.output().clone().into();
    for e in output {
        if let Some(id) = e.id() {
            root.push(id);
        }
    }
    root
}

pub fn tree_try_from_map(
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

// TODO: move this to try_form trait, once refactoring is completed
pub fn treelist_try_from_def(def: &Def) -> Result<Vec<Tree>, Error> {
    let map = InstrMap::from(def.clone());
    let mut res: Vec<Tree> = Vec::new();
    let roots = tree_roots_from_def(def);
    let mut visited: HashSet<Id> = HashSet::new();
    for r in roots {
        let tree = tree_try_from_map(&map, &mut visited, def.input(), &r, u64::MAX)?;
        res.push(tree);
    }
    Ok(res)
}

pub fn read_target_pat(prim: &str) -> pat::Target {
    let filename = format!("{}_pat.bin", prim);
    let path = Path::new(env!("OUT_DIR")).join(filename);
    pat::Target::deserialize_from_file(path)
}

pub fn read_target_imp(prim: &str) -> xim::Target {
    let filename = format!("{}_xim.bin", prim);
    let path = Path::new(env!("OUT_DIR")).join(filename);
    xim::Target::deserialize_from_file(path)
}

pub fn treemap_try_from_target(prim: &str) -> Result<TreeMap, Error> {
    let tpat = read_target_pat(prim);
    let timp = read_target_imp(prim);
    let mut tree_map = TreeMap::new();
    for (n, p) in tpat.pat() {
        if let Some(imp) = timp.get(n) {
            let cost = imp.perf();
            let instr_map = InstrMap::from(p.clone());
            let mut visited: HashSet<Id> = HashSet::new();
            let tree = tree_try_from_map(
                &instr_map,
                &mut visited,
                p.input(),
                &p.output().get_id(0)?,
                cost,
            )?;
            tree_map.insert(n.to_string(), tree);
        }
    }
    if tpat.pat().len() == timp.imp().len() && tree_map.len() == tpat.pat().len() {
        Ok(tree_map)
    } else {
        Err(Error::new_isel_error("missing a pattern"))
    }
}

pub fn treelist_try_from_prog(prog: &Prog) -> Result<Vec<Tree>, Error> {
    if let Some(main) = prog.get("main") {
        Ok(treelist_try_from_def(main)?)
    } else {
        Err(Error::new_isel_error("prog must have a main"))
    }
}
