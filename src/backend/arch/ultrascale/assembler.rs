use crate::backend::arch::ultrascale::isa;
use crate::backend::arch::ultrascale::prim::ast::{Gnd, Vcc};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;
use std::collections::{HashMap, HashSet};

pub trait Emit {
    fn emit(asm: &mut Assembler, instr: asm::Instr);
}

#[derive(Clone, Debug)]
pub struct Assembler {
    pub clock: String,
    pub reset: String,
    pub vcc: String,
    pub gnd: String,
    pub variable_map: HashMap<String, String>,
    pub variables: u32,
    pub num_instances: u32,
    pub ports: Vec<verilog::Port>,
    pub wires: Vec<verilog::Stmt>,
    pub instances: Vec<verilog::Stmt>,
    pub assignments: Vec<verilog::Stmt>,
    pub output_set: HashSet<String>,
}

impl Default for Assembler {
    fn default() -> Assembler {
        Assembler {
            clock: "clock".to_string(),
            reset: "reset".to_string(),
            vcc: "vcc".to_string(),
            gnd: "gnd".to_string(),
            variable_map: HashMap::new(),
            variables: 0,
            num_instances: 0,
            ports: Vec::new(),
            wires: Vec::new(),
            instances: Vec::new(),
            assignments: Vec::new(),
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

    pub fn vcc(&self) -> String {
        self.vcc.to_string()
    }

    pub fn gnd(&self) -> String {
        self.gnd.to_string()
    }

    pub fn ports(&self) -> &Vec<verilog::Port> {
        &self.ports
    }

    pub fn wires(&self) -> &Vec<verilog::Stmt> {
        &self.wires
    }

    pub fn instances(&self) -> &Vec<verilog::Stmt> {
        &self.instances
    }

    pub fn assignments(&self) -> &Vec<verilog::Stmt> {
        &self.assignments
    }

    pub fn new_instance_name(&mut self) -> String {
        let name = format!("i{}", self.num_instances);
        self.num_instances += 1;
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

    pub fn add_instance(&mut self, instance: verilog::Stmt) {
        self.instances.push(instance);
    }

    pub fn add_assignment(&mut self, assignment: verilog::Stmt) {
        self.assignments.push(assignment);
    }

    pub fn emit_clock_and_reset(&mut self) {
        self.ports.push(verilog::Port::new_input(&self.clock(), 1));
        self.ports.push(verilog::Port::new_input(&self.reset(), 1));
    }

    pub fn emit_vcc_and_gnd(&mut self) {
        let mut vcc = Vcc::default();
        let mut gnd = Gnd::default();
        vcc.set_output(&self.vcc());
        gnd.set_output(&self.gnd());
        self.add_instance(verilog::Stmt::from(vcc));
        self.add_instance(verilog::Stmt::from(gnd));
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
        self.emit_vcc_and_gnd();
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
                match instr.prim().op().as_ref() {
                    "lut_and_b_b_b" => isa::LutAndBBB::emit(self, instr.clone()),
                    "lut_or_b_b_b" => isa::LutOrBBB::emit(self, instr.clone()),
                    "lut_eq_b_i8_i8" => isa::LutEqBI8I8::emit(self, instr.clone()),
                    "lut_mux_i8_b_i8_i8" => isa::LutMuxI8BI8I8::emit(self, instr.clone()),
                    "lut_reg_mux_i8_b_i8_i8_b" => isa::LutRegMuxI8BI8I8B::emit(self, instr.clone()),
                    "lut_reg_i8_i8_b" => isa::LutRegI8I8B::emit(self, instr.clone()),
                    _ => (),
                }
            } else {
                match instr.std().op() {
                    asm::StdOp::Const => isa::Constant::emit(self, instr.clone()),
                    _ => self.add_assignment(verilog::Stmt::from(instr.std().clone())),
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
        for instance in self.instances().iter() {
            module.add_stmt(instance.clone());
        }
        for assignment in self.assignments().iter() {
            module.add_stmt(assignment.clone());
        }
        module
    }
}
