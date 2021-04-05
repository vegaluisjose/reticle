use pat::parser::Parser;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("lut_pat.bin");
    let pattern = Parser::parse_from_file("../../../examples/target/patterns/lut.pat").unwrap();
    pattern.serialize_to_file(path);
}
