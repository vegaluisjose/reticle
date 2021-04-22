use std::env;
use std::path::Path;

fn build_implementation(prim: &str) {
    use xim::parser::Parser;
    let out_dir = env::var("OUT_DIR").unwrap();
    let imp_path = format!(
        "../../../examples/target/implementations/ultrascale/{}.xim",
        prim
    );
    let bin_name = format!("{}_xim.bin", prim);
    let imp = Parser::parse_from_file(imp_path).unwrap();
    let bin_path = Path::new(&out_dir).join(bin_name);
    imp.serialize_to_file(bin_path);
}

fn main() {
    println!("cargo:rerun-if-changed=../../../examples/target/implementations/ultrascale/lut.xim");
    println!("cargo:rerun-if-changed=../../../examples/target/implementations/ultrascale/dsp.xim");
    build_implementation("lut");
    build_implementation("dsp");
}
