use reticle::compiler::tree::Tree;
use reticle::ir::parser::IRParser;
use reticle::util::errors::Error;
use std::convert::TryInto;

fn main() -> Result<(), Error> {
    let prog = IRParser::parse_from_file("examples/fsm.ir");
    if let Ok(p) = prog {
        if let Some(main) = p.get("main") {
            let forest: Vec<Tree> = main.clone().try_into()?;
            for tree in forest {
                println!("{}", tree);
            }
        }
    }
    Ok(())
}
