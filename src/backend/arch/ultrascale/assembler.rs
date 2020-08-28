use crate::backend::arch::ultrascale::isa;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;
use std::collections::HashMap;

pub trait Emit {
    fn emit(asm: &mut Assembler, instr: asm::Instr);
}

#[derive(Clone, Debug)]
pub struct Assembler {
    pub clock: String,
    pub reset: String,
    pub map: HashMap<String, String>,
    pub variables: u32,
    pub instances: u32,
    pub ports: Vec<verilog::Port>,
    pub wires: Vec<verilog::Stmt>,
    pub luts: Vec<verilog::Stmt>,
}

impl Default for Assembler {
    fn default() -> Assembler {
        Assembler {
            clock: "clock".to_string(),
            reset: "reset".to_string(),
            map: HashMap::new(),
            variables: 0,
            instances: 0,
            ports: Vec::new(),
            wires: Vec::new(),
            luts: Vec::new(),
        }
    }
}

// remove this once I add another instruction to match
#[allow(clippy::single_match)]
impl Assembler {
    pub fn clock(&self) -> String {
        self.clock.to_string()
    }
    pub fn reset(&self) -> String {
        self.reset.to_string()
    }
    pub fn ports(&self) -> &Vec<verilog::Port> {
        &self.ports
    }
    pub fn wires(&self) -> &Vec<verilog::Stmt> {
        &self.wires
    }
    pub fn luts(&self) -> &Vec<verilog::Stmt> {
        &self.luts
    }
    pub fn new_instance_name(&mut self) -> String {
        let name = format!("i{}", self.instances);
        self.instances += 1;
        name
    }
    pub fn new_variable_name(&mut self) -> String {
        let name = format!("t{}", self.variables);
        self.variables += 1;
        name
    }
    pub fn update_variable(&mut self, old: &str, new: &str) {
        self.map.insert(old.to_string(), new.to_string());
    }
    pub fn replace_variable(&mut self, name: &str) -> String {
        if let Some(var) = self.map.get(name) {
            var.to_string()
        } else {
            let tmp = self.new_variable_name();
            self.update_variable(name, &tmp);
            tmp
        }
    }
    pub fn add_wire(&mut self, wire: verilog::Stmt) {
        self.wires.push(wire);
    }
    pub fn add_lut(&mut self, lut: verilog::Stmt) {
        self.luts.push(lut);
    }
    pub fn emit_clock_and_reset(&mut self) {
        self.ports.push(verilog::Port::new_input(&self.clock(), 1));
        self.ports.push(verilog::Port::new_input(&self.reset(), 1));
    }
    pub fn emit_port(&mut self, port: asm::Port) {
        match port {
            asm::Port::Input { id, ty } => {
                if ty.is_vector() {
                    for i in 0..ty.length() {
                        let name = format!("{}_{}", id, i);
                        let port = verilog::Port::new_input(&name, ty.width());
                        self.ports.push(port);
                    }
                } else {
                    let port = verilog::Port::new_input(&id, ty.width());
                    self.ports.push(port);
                }
            }
            asm::Port::Output { id, ty } => {
                if ty.is_vector() {
                    for i in 0..ty.length() {
                        let name = format!("{}_{}", id, i);
                        let port = verilog::Port::new_output(&name, ty.width());
                        self.ports.push(port);
                    }
                } else {
                    let port = verilog::Port::new_output(&id, ty.width());
                    self.ports.push(port);
                }
            }
        }
    }
    pub fn emit(&mut self, prog: asm::Prog) -> verilog::Module {
        self.emit_clock_and_reset();
        for input in prog.inputs().iter() {
            self.emit_port(input.clone());
            self.update_variable(&input.id(), &input.id());
        }
        for output in prog.outputs().iter() {
            self.emit_port(output.clone());
            self.update_variable(&output.id(), &output.id());
        }
        for instr in prog.body().iter() {
            if instr.is_prim() {
                match instr.prim_op().as_ref() {
                    "lut_and_b_b_b" => isa::LutAndBBB::emit(self, instr.clone()),
                    _ => (),
                }
            }
        }
        let mut module = verilog::Module::new(&prog.id());
        for port in self.ports().iter() {
            module.add_port(port.clone());
        }
        for lut in self.luts().iter() {
            module.add_stmt(lut.clone());
        }
        module
    }
}
