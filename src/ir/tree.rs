use crate::ir::ast::*;
use crate::util::errors::Error;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;

pub fn find_tree_root(def: &Def) -> Vec<Id> {
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

#[derive(Clone, Debug)]
pub enum NodeOp {
    Wire(OpWire),
    Comp(OpComp),
    Inp,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub name: Id,
    pub op: NodeOp,
    pub attr: Option<Expr>,
    pub prim: Option<Prim>,
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
        let dst: Vec<Id> = input.dst().clone().try_into()?;
        if let Some(name) = dst.get(0) {
            let name = name.clone();
            let op = NodeOp::from(input.op().clone());
            let attr = Some(input.attr().clone());
            Ok(Node {
                name,
                op,
                attr,
                prim: None,
            })
        } else {
            Err(Error::new_conv_error("dst must have at least one term"))
        }
    }
}

impl TryFrom<InstrComp> for Node {
    type Error = Error;
    fn try_from(input: InstrComp) -> Result<Self, Self::Error> {
        let dst: Vec<Id> = input.dst().clone().try_into()?;
        if let Some(name) = dst.get(0) {
            let name = name.clone();
            let op = NodeOp::from(input.op().clone());
            let attr = Some(input.attr().clone());
            let prim = Some(input.prim().clone());
            Ok(Node {
                name,
                op,
                attr,
                prim,
            })
        } else {
            Err(Error::new_conv_error("dst must have at least one term"))
        }
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

// fn walk(map: &HashMap<Id, Instr>, visited: &HashSet<Id>, node: &Id) -> HashSet<Id> {
//     let mut visited = visited.clone();
//     let mut stack: Vec<Id> = Vec::new();
//     stack.push(node.clone());
//     visited.insert(node.clone());
//     while let Some(cur) = stack.pop() {
//         println!("visiting {}", &cur);
//         if let Some(instr) = map.get(&cur) {
//             let args: Vec<Id> = instr.arg().clone().try_into().unwrap();
//             for a in args {
//                 if !visited.contains(&a) {
//                     visited.insert(a.clone());
//                     stack.push(a);
//                 }
//             }
//         }
//     }
//     visited
// }

pub fn create_tree_from_def(def: &Def) {
    let mut map: HashMap<Id, Instr> = HashMap::new();
    // create instruction map
    for instr in def.body() {
        if let Some(term) = instr.dst().term() {
            if let Some(id) = term.id() {
                map.insert(id, instr.clone());
            }
        }
    }
    // let root = find_tree_root(def);
    // // add inputs as visited
    // let mut visited: HashSet<Id> = HashSet::new();
    // let input: Vec<ExprTerm> = def.input().clone().into();
    // for e in input {
    //     if let Some(id) = e.id() {
    //         visited.insert(id.clone());
    //     }
    // }
    // // walk
    // for r in root {
    //     println!("debug {}", &r);
    //     visited = walk(&map, &visited, &r);
    // }
}
