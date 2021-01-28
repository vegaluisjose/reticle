use reticle::tdl::parser::TDLParser;

fn main() {
    let target = TDLParser::parse_from_file("examples/ultrascale.tdl");
    println!("{}", target.unwrap())
    // if let Ok(t) = target {
    //     println!("{}", t);
    // }
}
