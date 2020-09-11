use crate::backend::arch::ultrascale::isa;
use crate::backend::arch::ultrascale::prim::ast::{Gnd, Vcc};
use crate::backend::asm::ast as asm;
use crate::backend::verilog;
use std::collections::{HashMap, HashSet};

pub trait Emit {
    fn emit(asm: &mut Assembler, instr: asm::Instr);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Name {
    Scalar(String),
    Vector(String, u64),
}

impl Name {
    pub fn new_scalar(id: &str) -> Name {
        Name::Scalar(id.to_string())
    }

    pub fn new_vector(id: &str, index: u64) -> Name {
        Name::Vector(id.to_string(), index)
    }

    pub fn id(&self) -> String {
        match self {
            Name::Scalar(id) => id.to_string(),
            Name::Vector(id, _) => id.to_string(),
        }
    }
}

fn emit_vector_index(name: &str, index: u64) -> String {
    format!("{}_{}", name, index)
}

#[derive(Clone, Debug)]
pub struct Assembler {
    pub clock: String,
    pub reset: String,
    pub vcc: String,
    pub gnd: String,
    pub variable_map: HashMap<Name, String>,
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

    pub fn update_scalar_variable(&mut self, old: &str, new: &str) {
        let name = Name::new_scalar(old);
        self.variable_map.insert(name, new.to_string());
    }

    pub fn update_vector_variable(&mut self, old_id: &str, old_index: u64, new: &str) {
        let name = Name::new_vector(old_id, old_index);
        self.variable_map.insert(name, new.to_string());
    }

    pub fn fresh_scalar_variable(&mut self, name: &str) -> String {
        let key = Name::new_scalar(name);
        if let Some(var) = self.variable_map.get(&key) {
            var.to_string()
        } else {
            let tmp = self.new_variable_name();
            self.update_scalar_variable(name, &tmp);
            tmp
        }
    }

    pub fn fresh_vector_variable(&mut self, name: &str, index: u64) -> String {
        let key = Name::new_vector(name, index);
        if let Some(var) = self.variable_map.get(&key) {
            var.to_string()
        } else {
            let tmp = self.new_variable_name();
            self.update_vector_variable(name, index, &tmp);
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
        let vcc_wire = verilog::Decl::new_wire(&self.vcc, 1);
        let gnd_wire = verilog::Decl::new_wire(&self.gnd, 1);
        self.add_wire(verilog::Stmt::from(vcc_wire));
        self.add_wire(verilog::Stmt::from(gnd_wire));
        self.add_instance(verilog::Stmt::from(vcc));
        self.add_instance(verilog::Stmt::from(gnd));
    }

    pub fn emit_wire(&mut self, expr: asm::Expr) {
        let width = expr.ty().width();
        if expr.ty().is_vector() {
            for i in 0..expr.ty().length() {
                let id = self.fresh_vector_variable(&expr.id(), i);
                let wire = verilog::Decl::new_wire(&id, width);
                self.add_wire(verilog::Stmt::from(wire));
            }
        } else {
            let id = self.fresh_scalar_variable(&expr.id());
            let wire = verilog::Decl::new_wire(&id, width);
            self.add_wire(verilog::Stmt::from(wire));
        }
    }

    pub fn emit_port(&mut self, port: asm::Port) {
        let width = port.ty().width();
        if port.ty().is_vector() {
            for i in 0..port.ty().length() {
                let name = emit_vector_index(&port.id(), i);
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
            if input.is_vector() {
                for i in 0..input.length() {
                    let name = emit_vector_index(&input.id(), i);
                    self.update_vector_variable(&input.id(), i, &name);
                }
            } else {
                self.update_scalar_variable(&input.id(), &input.id());
            }
        }
        for output in prog.outputs().iter() {
            self.emit_port(output.clone());
            if output.is_vector() {
                for i in 0..output.length() {
                    let name = emit_vector_index(&output.id(), i);
                    self.update_vector_variable(&output.id(), i, &name);
                }
            } else {
                self.update_scalar_variable(&output.id(), &output.id());
            }
            self.add_output(&output.id());
        }
        for instr in prog.body().iter() {
            if !self.is_output(&instr.dst().id()) {
                self.emit_wire(instr.dst().clone());
            }
            if instr.is_prim() {
                match instr.prim().op().as_ref() {
                    "lut_and_b_b_b" => isa::LutAnd::emit(self, instr.clone()),
                    "lut_or_b_b_b" => isa::LutOr::emit(self, instr.clone()),
                    "lut_eq_b_i8_i8" => isa::LutEq::emit(self, instr.clone()),
                    "lut_mux_i8_b_i8_i8" => isa::LutMux::emit(self, instr.clone()),
                    "lut_reg_mux_i8_b_i8_i8_b" => isa::LutMux::emit(self, instr.clone()),
                    "lut_reg_i8_i8_b" => isa::LutReg::emit(self, instr.clone()),
                    "dsp_add_i8v4_i8v4_i8v4" => isa::DspVector::emit(self, instr.clone()),
                    "dsp_add_reg_mul_i8_i8_i8_b_i8" => isa::DspScalar::emit(self, instr.clone()),
                    _ => unimplemented!(),
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
