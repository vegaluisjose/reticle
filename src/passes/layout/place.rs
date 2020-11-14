use crate::asm::ast::{InstrPhy, Prim, Prog};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PlacerInput {
    pub id: u32,
    pub prim: Prim,
}

impl PlacerInput {
    pub fn new(id: u32, prim: Prim) -> PlacerInput {
        PlacerInput { id, prim }
    }
}

#[derive(Clone, Debug)]
pub struct Placer {
    pub counter: u32,
    pub var: HashMap<String, u32>,
    pub num: HashMap<u32, String>,
    pub inputs: Vec<PlacerInput>,
}

impl Default for Placer {
    fn default() -> Self {
        Placer {
            counter: 0,
            var: HashMap::new(),
            num: HashMap::new(),
            inputs: Vec::new(),
        }
    }
}

impl Placer {
    pub fn counter(&self) -> u32 {
        self.counter
    }

    pub fn inputs(&self) -> &Vec<PlacerInput> {
        &self.inputs
    }

    pub fn rename(&mut self, current: &str) -> u32 {
        let value = self.counter();
        self.counter += 1;
        self.var.insert(current.to_string(), value);
        self.num.insert(value, current.to_string());
        value
    }

    pub fn add_input(&mut self, id: u32, prim: Prim) {
        let input = PlacerInput::new(id, prim);
        self.inputs.push(input);
    }

    pub fn add_instr(&mut self, instr: &InstrPhy) {
        let id = self.rename(&instr.dst().id());
        self.add_input(id, instr.loc().prim().clone());
    }

    pub fn run(&mut self) {
        for i in self.inputs() {
            println!("{:?}", i);
        }
    }
}

pub fn place_basic(prog: &Prog) {
    let mut placer = Placer::default();
    for instr in prog.body() {
        if instr.is_phy() {
            placer.add_instr(instr.phy());
        }
    }
    placer.run();
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
