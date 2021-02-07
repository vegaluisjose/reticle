use crate::ir::ast::*;
use crate::tdl::ast::Pat;
use crate::util::errors::Error;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt;

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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum NodeOp {
    Wire(OpWire),
    Comp(OpComp),
    Inp,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub index: u64,
    pub id: Id,
    pub ty: Ty,
    pub op: NodeOp,
    pub attr: Expr,
    pub prim: Prim,
    pub cost: u64,
    pub fixed: bool,
    pub pat: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Tree {
    pub index: u64,
    pub node: HashMap<u64, Node>,
    pub edge: HashMap<u64, Vec<u64>>,
}

#[derive(Clone, Debug, Default)]
pub struct Forest {
    pub visited: HashSet<Id>,
    pub tree: Vec<Tree>,
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

impl Default for Tree {
    fn default() -> Self {
        Tree {
            index: 0,
            node: HashMap::new(),
            edge: HashMap::new(),
        }
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

impl fmt::Display for NodeOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeOp::Wire(wire) => write!(f, "{}", wire),
            NodeOp::Comp(comp) => write!(f, "{}", comp),
            NodeOp::Inp => write!(f, "inp"),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}-{}",
            self.ty(),
            self.op(),
            self.attr(),
            self.prim()
        )
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut digraph = String::from("digraph {\n");
        // declare nodes
        for i in 0..self.index {
            if let Some(node) = self.node.get(&i) {
                let label = format!("{} [ label = \"{}\" ]\n", i, node);
                digraph.push_str(&label);
            }
        }
        // declare edges
        for i in 0..self.index {
            if let Some(edges) = self.edge.get(&i) {
                for e in edges {
                    let edge = format!("{} -> {} [ ]\n", i, e);
                    digraph.push_str(&edge);
                }
            }
        }
        digraph.push('}');
        write!(f, "{}", digraph)
    }
}

impl From<OpWire> for NodeOp {
    fn from(input: OpWire) -> Self {
        NodeOp::Wire(input)
    }
}

impl From<OpComp> for NodeOp {
    fn from(input: OpComp) -> Self {
        NodeOp::Comp(input)
    }
}

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
