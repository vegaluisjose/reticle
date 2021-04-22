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

fn main() {
    println!("cargo:rerun-if-changed=../../../examples/pat/lut.pat");
    println!("cargo:rerun-if-changed=../../../examples/pat/dsp.pat");
    println!("cargo:rerun-if-changed=../../../examples/xim/lut.xim");
    println!("cargo:rerun-if-changed=../../../examples/xim/dsp.xim");
    build_pattern("lut");
    build_pattern("dsp");
    build_implementation("lut");
    build_implementation("dsp");
}
