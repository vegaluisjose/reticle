use crate::asm::ast::{InstrPhy, Prim, Prog};
use crate::util::file::{create_absolute, read_from_tempfile, remove_tempfile, write_to_tempfile};
use std::collections::HashMap;
use std::fmt;
use std::process::Command;

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

impl fmt::Display for PlacerInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.id, self.prim)
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
        let bin = create_absolute("layout/place.py");
        let filename = "__reticle_locations.txt";
        let contents: String = self.inputs().iter().map(|i| format!("{}\n", i)).collect();
        let filepath = write_to_tempfile(&filename, &contents);
        let output = Command::new("python3")
            .arg(bin)
            .arg(&filepath)
            .output()
            .expect("failed to execute place.py");
        println!("{}", read_from_tempfile(&filepath));
        println!(
            "solution:\n{}",
            String::from_utf8_lossy(&output.stdout).to_string()
        );
        remove_tempfile(filepath);
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
