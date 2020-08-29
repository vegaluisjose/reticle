use crate::backend::arch::ultrascale::isa;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;
use std::collections::HashMap;

pub trait EmitPrim {
    fn emit_prim(asm: &mut Assembler, instr: asm::InstrPrim);
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
    pub fn fresh_variable(&mut self, name: &str) -> String {
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
    pub fn emit_wire(&mut self, expr: asm::Expr) {
        if expr.is_ref() {
            let width = expr.ty().width();
            if expr.ty().is_vector() {
                for i in 0..expr.ty().length() {
                    let name = format!("{}_{}", expr.id(), i);
                    let wire = verilog::Decl::new_wire(&name, width);
                    self.add_wire(verilog::Stmt::from(wire));
                }
            } else {
                let wire = verilog::Decl::new_wire(&expr.id(), width);
                self.add_wire(verilog::Stmt::from(wire));
            }
        } else {
            panic!("Error: only reference expr wire conversion allowed")
        }
    }
    pub fn emit_port(&mut self, port: asm::Port) {
        let width = port.ty().width();
        if port.ty().is_vector() {
            for i in 0..port.ty().length() {
                let name = format!("{}_{}", port.id(), i);
                let vport = if port.is_input() {
                    verilog::Port::new_input(&name, width)
                } else {
                    verilog::Port::new_output(&name, width)
                };
                self.ports.push(vport);
            }
        } else {
            let vport = if port.is_input() {
                verilog::Port::new_input(&port.id(), width)
            } else {
                verilog::Port::new_output(&port.id(), width)
            };
            self.ports.push(vport);
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
                let prim = instr.prim();
                match prim.op().as_ref() {
                    "lut_and_b_b_b" => isa::LutAndBBB::emit_prim(self, prim.clone()),
                    "lut_eq_b_i8_i8" => isa::LutEqI8I8::emit_prim(self, prim.clone()),
                    _ => (),
                }
            }
        }
        let mut module = verilog::Module::new(&prog.id());
        for port in self.ports().iter() {
            module.add_port(port.clone());
        }
        for wire in self.wires().iter() {
            module.add_stmt(wire.clone());
        }
        for lut in self.luts().iter() {
            module.add_stmt(lut.clone());
        }
        module
    }
}
