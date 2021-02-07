use reticle::compiler::tree::Forest;
use reticle::ir::parser::IRParser;
use std::convert::TryFrom;

fn main() {
    let prog = IRParser::parse_from_file("examples/fsm.ir");
    if let Ok(p) = prog {
        if let Some(d) = p.get("main") {
            let forest = Forest::try_from(d.clone()).unwrap();
            for tree in forest.tree() {
                println!("{}", tree);
            }
        }
    }
}
