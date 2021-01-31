use reticle::ir::partition::Tree;
use reticle::tdl::parser::TDLParser;
use std::convert::TryFrom;

fn main() {
    let target = TDLParser::parse_from_file("examples/ultrascale.tdl");
    if let Ok(t) = target {
        for p in t.pat().values() {
            if let Ok(tree) = Tree::try_from(p.clone()) {
                println!("{}", tree);
            }
        }
    }
}
