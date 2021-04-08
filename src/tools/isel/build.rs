use std::env;
use std::path::Path;

fn build_pattern(prim: &str) {
    use pat::parser::Parser;
    let out_dir = env::var("OUT_DIR").unwrap();
    let pat_path = format!("../../../examples/target/patterns/{}.pat", prim);
    let bin_name = format!("{}_pat.bin", prim);
    let pat = Parser::parse_from_file(pat_path).unwrap();
    let bin_path = Path::new(&out_dir).join(bin_name);
    pat.serialize_to_file(bin_path);
}

fn build_implementation(prim: &str) {
    use xim::parser::Parser;
    let out_dir = env::var("OUT_DIR").unwrap();
    let imp_path = format!("../../../examples/target/implementations/ultrascale/{}.xim", prim);
    let bin_name = format!("{}_xim.bin", prim);
    let imp = Parser::parse_from_file(imp_path).unwrap();
    let bin_path = Path::new(&out_dir).join(bin_name);
    imp.serialize_to_file(bin_path);
}

fn main() {
    build_pattern("lut");
    build_pattern("dsp");
    build_implementation("lut");
    build_implementation("dsp");
}
