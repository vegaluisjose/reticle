use reticle::ir::parser::IRParser;
use reticle::tdl::parser::TDLParser;
use reticle::ir::partition::{Forest, Tree};
use reticle::util::errors::Error;
use std::convert::TryFrom;

// fn tree_from_pats(file: &str) -> Result<Vec<(String, Tree)>, Error> {
//     let mut pattern: Vec<(String, Tree)> = Vec::new();
//     let target = TDLParser::parse_from_file(file)?;
//     for (n, p) in target.pat() {
//         pattern.push((n.clone(), Tree::try_from(p.clone())?));
//     }
//     Ok(pattern)
// }

fn tree_from_prog(file: &str) -> Result<Vec<Tree>, Error> {
    let prog = IRParser::parse_from_file(file)?;
    if let Some(main) = prog.get("main") {
        let forest = Forest::try_from(main.clone())?;
        Ok(forest.tree().clone())
    } else {
        Err(Error::new_conv_error("converting to tree"))
    }
}


fn main() -> Result<(), Error> {
    // let pats = tree_from_pats("examples/ultrascale.tdl")?;
    let blks = tree_from_prog("examples/fsm.ir")?;
    for b in blks {
        let cuts = b.cut(0);
        for c in cuts {
            if let Some(node) = b.node(c) {
                println!("start node:{}", node);
                let cindex = b.dfg(c);
                for ci in cindex {
                    if let Some(vnode) = b.node(ci) {
                        println!("{}", vnode);
                    }
                }
                println!("\n")
            }
            // let ci = b.dfg(c);
            // for (n, p) in &pats {
            //     println!("Checking pat: {}", n);
            //     let pi = p.dfg(0);
            //     for (pii, cii) in pi.iter().zip(ci.iter()) {
            //         if let Some(pnode) = p.node(*pii) {
            //             if let Some(bnode) = b.node(*cii) {
            //                 println!("pnode:{}   cnode:{}", pnode, bnode);
            //             }
            //         }
            //     }
            // }
        }
    }
    Ok(())
}
