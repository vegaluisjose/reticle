use crate::compiler::tree::*;
use crate::ir::ast::*;
use crate::util::errors::Error;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::convert::TryFrom;

pub fn find_roots(def: &Def) -> Vec<Id> {
    let mut count: HashMap<Id, u64> = HashMap::new();
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
    pub fn is_comp(&self) -> bool {
        matches!(self.op, NodeOp::Comp(_))
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
    pub fn is_fixed(&self) -> bool {
        self.fixed
    }
    pub fn set_index(&mut self, index: u64) {
        self.index = index;
    }
    pub fn set_cost(&mut self, cost: u64) {
        self.cost = cost;
    }
    pub fn set_fixed(&mut self) {
        self.fixed = true;
    }
    pub fn set_pat(&mut self, name: &str) {
        self.pat = Some(name.to_string());
    }
    pub fn clear_fixed(&mut self) {
        self.fixed = false;
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
                if node.is_comp() && !node.is_fixed() {
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
            fixed: false,
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
}

impl Forest {
    pub fn was_visited(&self, id: &str) -> bool {
        self.visited.contains(id)
    }
    pub fn tree(&self) -> &Vec<Tree> {
        &self.tree
    }
    pub fn add_visited(&mut self, name: &str) {
        if !self.visited.contains(name) {
            self.visited.insert(name.to_string());
        }
    }
    pub fn add_tree(&mut self, tree: Tree) {
        self.tree.push(tree);
    }
}
