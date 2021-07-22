use std::env;
use std::path::Path;

fn build_pattern(prim: &str) {
    use pat::parser::Parser;
    let out_dir = env::var("OUT_DIR").unwrap();
    let pat_path = format!("../../../examples/pat/{}.pat", prim);
    let bin_name = format!("{}_pat.bin", prim);
    let pat = Parser::parse_from_file(pat_path).unwrap();
    let bin_path = Path::new(&out_dir).join(bin_name);
    pat.serialize_to_file(bin_path);
}

fn build_implementation(prim: &str) {
    use xim::parser::Parser;
    let out_dir = env::var("OUT_DIR").unwrap();
    let imp_path = format!("../../../examples/xim/{}.xim", prim);
    let bin_name = format!("{}_xim.bin", prim);
    let imp = Parser::parse_from_file(imp_path).unwrap();
    let bin_path = Path::new(&out_dir).join(bin_name);
    imp.serialize_to_file(bin_path);
}

fn build(prim: &str) {
    println!("cargo:rerun-if-changed=../../../examples/pat/{}.pat", prim);
    println!("cargo:rerun-if-changed=../../../examples/xim/{}.xim", prim);
    build_pattern(prim);
    build_implementation(prim);
}

fn main() {
    let prim = ["lut", "dsp", "mem"];
    for p in &prim {
        build(p);
    }
}
