use crate::asm::ast::{InstrPhy, Prim, Prog};
use crate::util::file::{create_absolute, remove_tempfile, write_to_tempfile};
use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;
use std::process::Command;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct PlacerInput {
    pub id: u32,
    pub prim: Prim,
}

#[derive(Clone, Debug)]
pub struct PlacerOutput {
    pub id: u32,
    pub x: u32,
    pub y: u32,
}

impl FromStr for PlacerOutput {
    type Err = ParseIntError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = input.split(",").collect();
        match values.len() {
            3 => {
                let id = values[0].parse::<u32>()?;
                let x = values[1].parse::<u32>()?;
                let y = values[2].parse::<u32>()?;
                Ok(PlacerOutput::new(id, x, y))
            }
            _ => panic!("Error: {} is not valid placer output format", input),
        }
    }
}

impl PlacerInput {
    pub fn new(id: u32, prim: Prim) -> PlacerInput {
        PlacerInput { id, prim }
    }
}

impl PlacerOutput {
    pub fn new(id: u32, x: u32, y: u32) -> PlacerOutput {
        PlacerOutput { id, x, y }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}

impl fmt::Display for PlacerInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.id, self.prim)
    }
}

impl fmt::Display for PlacerOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.id(), self.x(), self.y())
    }
}

#[derive(Clone, Debug)]
pub struct Placer {
    pub counter: u32,
    pub var: HashMap<String, u32>,
    pub num: HashMap<u32, String>,
    pub inputs: Vec<PlacerInput>,
    pub outputs: HashMap<String, PlacerOutput>,
}

impl Default for Placer {
    fn default() -> Self {
        Placer {
            counter: 0,
            var: HashMap::new(),
            num: HashMap::new(),
            inputs: Vec::new(),
            outputs: HashMap::new(),
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

    pub fn lookup_num(&self, key: u32) -> Option<&String> {
        self.num.get(&key)
    }

    pub fn lookup_output(&self, key: &str) -> Option<&PlacerOutput> {
        self.outputs.get(key)
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

    pub fn add_output(&mut self, input: &str) {
        let output = PlacerOutput::from_str(input).expect("Error parsing placer output");
        let id = if let Some(n) = self.lookup_num(output.id()) {
            n.to_string()
        } else {
            panic!("Error: key not found")
        };
        self.outputs.insert(id, output);
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
        let res = String::from_utf8_lossy(&output.stdout).to_string();
        for i in res.lines() {
            self.add_output(i);
        }
        for (k, v) in self.outputs.clone() {
            println!("{} --> {}", k, v);
        }
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
