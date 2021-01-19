use reticle::ir::parser::IRParser;

fn main() {
    let prog = IRParser::parse_from_file("examples/muladd_reg.ir");
    if let Ok(p) = prog {
        if let Some(def) = p.get("main") {
            let mut test = def.clone();
            println!("{}", &test);
            test.shuffle_body();
            println!("{}", &test);
        }
    }
}
