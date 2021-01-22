// use reticle::ir::dfg::Dfg;
use reticle::ir::ast as ir;
use reticle::ir::parser::IRParser;
// use std::convert::TryFrom;
// use petgraph::visit::Dfs;
use std::collections::{HashMap, HashSet};

fn find_roots(def: &ir::Def) {
    let mut count: HashMap<ir::Id, u64> = HashMap::new();
    let mut root: HashSet<ir::Id> = HashSet::new();
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
    for (k, v) in count {
        if v != 1 {
            root.insert(k);
        }
    }
    for r in root {
        println!("root --> {}", r);
    }
}

fn main() {
    let prog = IRParser::parse_from_file("examples/fsm.ir");
    if let Ok(p) = prog {
        if let Some(d) = p.get("main") {
            find_roots(d);
        }
    }
}
