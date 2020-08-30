use crate::backend::arch::ultrascale::isa;
use crate::backend::asm::ast as asm;
use crate::backend::verilog;
use std::collections::{HashMap, HashSet};

pub trait EmitPrim {
    fn emit_prim(asm: &mut Assembler, instr: asm::InstrPrim);
}

#[derive(Clone, Debug)]
pub struct Assembler {
    pub clock: String,
    pub reset: String,
    pub variable_map: HashMap<String, String>,
    pub variables: u32,
    pub instances: u32,
    pub ports: Vec<verilog::Port>,
    pub wires: Vec<verilog::Stmt>,
    pub regs: Vec<verilog::Stmt>,
    pub luts: Vec<verilog::Stmt>,
    pub stds: Vec<verilog::Stmt>,
    pub output_set: HashSet<String>,
}

impl Default for Assembler {
    fn default() -> Assembler {
        Assembler {
            clock: "clock".to_string(),
            reset: "reset".to_string(),
            variable_map: HashMap::new(),
            variables: 0,
            instances: 0,
            ports: Vec::new(),
            wires: Vec::new(),
            regs: Vec::new(),
            luts: Vec::new(),
            stds: Vec::new(),
            output_set: HashSet::new(),
        }
    }
}

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

    pub fn regs(&self) -> &Vec<verilog::Stmt> {
        &self.regs
    }

    pub fn stds(&self) -> &Vec<verilog::Stmt> {
        &self.stds
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
        self.variable_map.insert(old.to_string(), new.to_string());
    }

    pub fn fresh_variable(&mut self, name: &str) -> String {
        if let Some(var) = self.variable_map.get(name) {
            var.to_string()
        } else {
            let tmp = self.new_variable_name();
            self.update_variable(name, &tmp);
            tmp
        }
    }

    pub fn add_output(&mut self, name: &str) {
        self.output_set.insert(name.to_string());
    }

    pub fn is_output(&mut self, name: &str) -> bool {
        self.output_set.contains(name)
    }

    pub fn add_wire(&mut self, wire: verilog::Stmt) {
        self.wires.push(wire);
    }

    pub fn add_reg(&mut self, reg: verilog::Stmt) {
        self.regs.push(reg);
    }

    pub fn add_lut(&mut self, lut: verilog::Stmt) {
        self.luts.push(lut);
    }

    pub fn add_std(&mut self, std: verilog::Stmt) {
        self.stds.push(std);
    }

    pub fn emit_clock_and_reset(&mut self) {
        self.ports.push(verilog::Port::new_input(&self.clock(), 1));
        self.ports.push(verilog::Port::new_input(&self.reset(), 1));
    }

    pub fn emit_wire(&mut self, expr: asm::Expr) {
        let width = expr.ty().width();
        let id = self.fresh_variable(&expr.id());
        if expr.ty().is_vector() {
            for i in 0..expr.ty().length() {
                let name = format!("{}_{}", &id, i);
                let wire = verilog::Decl::new_wire(&name, width);
                self.add_wire(verilog::Stmt::from(wire));
            }
        } else {
            let wire = verilog::Decl::new_wire(&id, width);
            self.add_wire(verilog::Stmt::from(wire));
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
            self.add_output(&output.id());
        }
        for instr in prog.body().iter() {
            if !self.is_output(&instr.dst().id()) {
                self.emit_wire(instr.dst().clone());
            }
            if instr.is_prim() {
                let prim = instr.prim();
                match prim.op().as_ref() {
                    "lut_and_b_b_b" => isa::LutAndBBB::emit_prim(self, prim.clone()),
                    "lut_or_b_b_b" => isa::LutOrBBB::emit_prim(self, prim.clone()),
                    "lut_eq_b_i8_i8" => isa::LutEqBI8I8::emit_prim(self, prim.clone()),
                    "lut_mux_i8_b_i8_i8" => isa::LutMuxI8BI8I8::emit_prim(self, prim.clone()),
                    "lut_reg_mux_i8_b_i8_i8_b" => {
                        isa::LutRegMuxI8BI8I8B::emit_prim(self, prim.clone())
                    }
                    _ => (),
                }
            } else {
                self.add_std(verilog::Stmt::from(instr.std().clone()));
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
        for reg in self.regs().iter() {
            module.add_stmt(reg.clone());
        }
        for std in self.stds().iter() {
            module.add_stmt(std.clone());
        }
        module
    }
}
