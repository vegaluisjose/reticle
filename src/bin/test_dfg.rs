use reticle::ir::dfg::Dfg;
use reticle::ir::parser::IRParser;
use reticle::ir::tree::create_tree_from_def;
use std::convert::TryFrom;

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
