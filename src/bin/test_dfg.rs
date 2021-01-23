// use reticle::ir::dfg::Dfg;
use reticle::ir::ast as ir;
use reticle::ir::parser::IRParser;
// use std::convert::TryFrom;
// use petgraph::visit::Dfs;
use std::collections::{HashMap, HashSet};

fn find_tree_root(def: &ir::Def) -> HashSet<ir::Id> {
    let mut count: HashMap<ir::Id, u64> = HashMap::new();
    let mut root: HashSet<ir::Id> = HashSet::new();
    // store compute instructions
    for instr in def.body() {
        if instr.is_comp() {
            let dst: Vec<ir::ExprTerm> = instr.dst().clone().into();
            for e in dst {
                if let Some(id) = e.id() {
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
    // a node is considered a root if:
    // a node is an output (v=0)
    // a node is used more than one time (v>1)
    for (k, v) in count {
        if v != 1 {
            root.insert(k);
        }
    }
    // add the outputs that are not compute instructions as roots
    let output: Vec<ir::ExprTerm> = def.output().clone().into();
    for e in output {
        if let Some(id) = e.id() {
            if !root.contains(&id) {
                root.insert(id);
            }
        }
    }
    root
}

fn main() {
    let prog = IRParser::parse_from_file("examples/fsm.ir");
    if let Ok(p) = prog {
        if let Some(d) = p.get("main") {
            let root = find_tree_root(d);
            for r in root {
                println!("root --> {}", r);
            }
        }
    }
}
