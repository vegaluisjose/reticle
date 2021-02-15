use reticle::asm::parser::AsmParser;

fn main() {
    let prog = AsmParser::parse_from_file("examples/lut_i8_add_ro.rasm");
    if let Ok(p) = prog {
        println!("{}", &p);
    }
}
