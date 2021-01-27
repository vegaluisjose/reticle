use reticle::ir::dfg::Dfg;
use reticle::ir::parser::IRParser;
use reticle::ir::tree::Forest;
use std::convert::TryFrom;

fn main() {
    let prog = IRParser::parse_from_file("examples/fsm.ir");
    if let Ok(p) = prog {
        if let Some(d) = p.get("main") {
            println!("{}", Dfg::try_from(d.clone()).unwrap());
            Forest::try_from(d.clone()).unwrap();
            // let root = find_tree_root(d);
            // for r in root {
            //     println!("root --> {}", r);
            // }
        }
    }
}
