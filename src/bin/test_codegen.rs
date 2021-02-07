use itertools::izip;
use reticle::codegen::partition::{Forest, Tree};
use reticle::ir::parser::IRParser;
use reticle::tdl::parser::TDLParser;
use reticle::util::errors::Error;
use std::convert::TryFrom;

fn tree_from_pats(file: &str) -> Result<Vec<(String, Tree)>, Error> {
    let mut pattern: Vec<(String, Tree)> = Vec::new();
    let target = TDLParser::parse_from_file(file)?;
    for (n, p) in target.pat() {
        pattern.push((n.clone(), Tree::try_from(p.clone())?));
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
    let bindex = block.bfs(start);
    let pindex = pat.bfs(0);
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

fn main() -> Result<(), Error> {
    let pats = tree_from_pats("examples/ultrascale.tdl")?;
    let blks = tree_from_prog("examples/fsm.ir")?;
    for btree in blks {
        let cuts = btree.cut(0);
        for cut in cuts {
            let mut ctree = btree.clone();
            for (pname, ptree) in &pats {
                let (is_valid, cost) = is_valid_change(&ctree, &ptree, cut);
                if is_valid {
                    if let Some(node) = ctree.node_mut(cut) {
                        println!("found one, cost:{} with name:{}", cost, &pname);
                        node.set_cost(cost);
                        node.set_pat(&pname);
                    }
                }
            }
        }
    }
    Ok(())
}
