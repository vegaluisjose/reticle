use reticle::ir::dfg::Dfg;
use reticle::ir::ast as ir;
use reticle::ir::parser::IRParser;
use std::convert::TryFrom;
// use petgraph::visit::Dfs;
use std::convert::TryInto;
use std::collections::{HashMap, HashSet};

fn find_tree_root(def: &ir::Def) -> Vec<ir::Id> {
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

fn walk(map: &HashMap<ir::Id, ir::Instr>, visited: &HashSet<ir::Id>, node: &ir::Id) -> HashSet<ir::Id> {
    let mut visited = visited.clone();
    let mut stack: Vec<ir::Id> = Vec::new();
    stack.push(node.clone());
    visited.insert(node.clone());
    while let Some(cur) = stack.pop() {
        println!("visiting {}", &cur);
        if let Some(instr) = map.get(&cur) {
            let args: Vec<ir::Id> = instr.arg().clone().try_into().unwrap();
            for a in args {
                if !visited.contains(&a) {
                    visited.insert(a.clone());
                    stack.push(a);
                }
            }
        }
    }
    visited
}

fn create_tree_from_def(def: &ir::Def) {
    let mut map: HashMap<ir::Id, ir::Instr> = HashMap::new();
    // create instruction map
    for instr in def.body() {
        if let Some(term) = instr.dst().term() {
            if let Some(id) = term.id() {
                map.insert(id, instr.clone());
            }
        }
    }
    let root = find_tree_root(def);
    // add inputs as visited
    let mut visited: HashSet<ir::Id> = HashSet::new();
    let input: Vec<ir::ExprTerm> = def.input().clone().into();
    for e in input {
        if let Some(id) = e.id() {
            visited.insert(id.clone());
        }
    }
    // walk
    for r in root {
        println!("debug {}", &r);
        visited = walk(&map, &visited, &r);
    }
}

fn main() {
    let prog = IRParser::parse_from_file("examples/fsm.ir");
    if let Ok(p) = prog {
        if let Some(d) = p.get("main") {
            println!("{}", Dfg::try_from(d.clone()).unwrap());
            create_tree_from_def(d);
            // let root = find_tree_root(d);
            // for r in root {
            //     println!("root --> {}", r);
            // }
        }
    }
}
