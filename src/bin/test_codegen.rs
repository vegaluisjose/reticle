use itertools::izip;
use reticle::ir::parser::IRParser;
use reticle::ir::partition::{Forest, Tree};
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

fn is_match(block: &Tree, pat: &Tree, start: u64) -> bool {
    if let Some(node) = block.node(start) {
        println!("index: {} node: {}", start, node);
    }
    let bindex = block.bfs(start);
    let pindex = pat.bfs(0);
    let mut is_match = true;
    for (p, b) in izip!(&pindex, &bindex) {
        if let Some(pnode) = pat.node(*p) {
            if let Some(bnode) = block.node(*b) {
                if pnode.ty() != bnode.ty() {
                    is_match = false;
                }
            }
        }
    }
    is_match
}

fn main() -> Result<(), Error> {
    let pats = tree_from_pats("examples/ultrascale.tdl")?;
    let blks = tree_from_prog("examples/fsm.ir")?;
    for btree in blks {
        let cuts = btree.cut(0);
        for cut in cuts {
            for (pname, ptree) in &pats {
                println!("pat:{} is_match:{}\n", pname, is_match(&btree, &ptree, cut));
            }
        }
    }
    Ok(())
}
