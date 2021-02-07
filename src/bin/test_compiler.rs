use itertools::izip;
use reticle::compiler::tree::{Forest, Tree};
use reticle::ir::parser::IRParser;
use reticle::tdl::ast::Pat;
use reticle::tdl::parser::TDLParser;
use reticle::util::errors::Error;
use std::collections::HashMap;
use std::convert::TryFrom;

fn tree_from_pats(pat_map: &HashMap<String, Pat>) -> Result<HashMap<String, Tree>, Error> {
    let mut pattern: HashMap<String, Tree> = HashMap::new();
    for (n, p) in pat_map {
        pattern.insert(n.clone(), Tree::try_from(p.clone())?);
    }
    Ok(pattern)
}

fn tree_from_prog(file: &str) -> Result<Vec<Tree>, Error> {
    let prog = IRParser::parse_from_file(file)?;
    if let Some(main) = prog.get("main") {
        let forest = Forest::try_from(main.clone())?;
        Ok(forest.tree().clone())
    } else {
        Err(Error::new_conv_error("converting to tree"))
    }
}

fn is_valid_change(block: &Tree, pat: &Tree, start: u64) -> (bool, u64) {
    let pindex = pat.bfs(0);
    let bindex = block.bfs_bound(start, pindex.len());
    let mut is_match = true;
    let mut bcost: u64 = 0;
    if let Some(proot) = pat.node(0) {
        let pcost = proot.cost();
        for (p, b) in izip!(&pindex, &bindex) {
            if let Some(pnode) = pat.node(*p) {
                if let Some(bnode) = block.node(*b) {
                    if pnode.ty() != bnode.ty()
                        || (!pnode.is_inp() && pnode.op() != bnode.op())
                        || (!pnode.is_inp()
                            && !bnode.prim().is_any()
                            && pnode.prim() != bnode.prim())
                        || (!pnode.is_inp() && pnode.attr() != bnode.attr())
                    {
                        is_match = false;
                    }
                    if !pnode.is_inp() && bcost != u64::MAX {
                        bcost += bnode.cost();
                    }
                }
            }
        }
        (is_match & (pcost < bcost), pcost)
    } else {
        (false, u64::MAX)
    }
}

fn tree_update(block: &Tree, pat: &Tree, target: u64, pat_name: &str, pat_cost: u64) -> Tree {
    let pindex = pat.bfs(0);
    let bindex = block.bfs_bound(target, pindex.len());
    let mut btree = block.clone();
    for (p, b) in izip!(&pindex, &bindex) {
        if let Some(pnode) = pat.node(*p) {
            if !pnode.is_inp() {
                if let Some(bnode) = btree.node_mut(*b) {
                    bnode.clear_pat();
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

fn tree_codegen(block: &Tree) {
    for index in block.dfg(0) {
        if let Some(node) = block.node(index) {
            if let Some(name) = node.pat() {
                println!("{}", name);
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let blks = tree_from_prog("examples/fsm.ir")?;
    let target = TDLParser::parse_from_file("examples/ultrascale.tdl")?;
    let pats = tree_from_pats(target.pat())?;
    for btree in blks {
        let mut ctree = btree.clone();
        let cuts = btree.cut(0);
        for cut in cuts {
            for (pname, ptree) in &pats {
                let (is_valid, cost) = is_valid_change(&ctree, &ptree, cut);
                if is_valid {
                    ctree = tree_update(&ctree, &ptree, cut, &pname, cost);
                }
            }
        }
        tree_codegen(&ctree);
    }
    Ok(())
}
