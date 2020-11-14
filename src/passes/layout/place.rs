use crate::asm::ast::Prog;

pub fn place_basic(prog: &Prog) {
    for instr in prog.body() {
        println!("{}", instr.dst().id());
    }
}

// use reticle::util::file::{create_absolute, write_to_tempfile};
// use std::path::Path;
// use std::process::Command;

// fn place<P: AsRef<Path>>(input: P) -> String {
//     let bin = create_absolute("layout/place.py");
//     let output = Command::new("python3")
//         .arg(bin)
//         .arg(input.as_ref())
//         .output()
//         .expect("failed to execute place.py");
//     String::from_utf8_lossy(&output.stdout).to_string()
// }

// fn main() {
// let constraints = write_to_tempfile("__reticle_constraints.txt", "0 dsp");
// println!("{:?}", &constraints);
// let x = place(constraints);
// println!("{}", x);
// }
