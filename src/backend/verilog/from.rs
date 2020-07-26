use crate::backend::asm::ast as asm;
use crate::backend::verilog::*;
use std::convert::From;

// there is going to be a better way to do this
// perhaps every primtive i.e., LUT, DSP, etc
// will have its own object
impl From<asm::Instr> for Instance {
    fn from(instr: asm::Instr) -> Self {
        if instr.op == "dsp_i8_add_reg_mul_lut_lut_lut_lut" {
            let mut inst = Instance::default();
            inst.set_prim("dsp_i8_add_reg_mul_lut_lut_lut_lut");
            // connect clock
            inst.connect_ref("CLK", "clock");
            // connect reset
            inst.connect_ref("RSTM", "reset");
            // connect inputs
            let inputs = vec!["A", "B", "CEM", "C"];
            for (input, param) in inputs.iter().zip(instr.params().iter()) {
                inst.connect_ref(input, &param.id());
            }
            // connect output
            inst.connect_ref("P", &instr.dst());
            inst
        } else {
            panic!("Error: unsupported op")
        }
    }
}

#[derive(Clone, Debug)]
pub struct Name {
    prefix: String,
    counter: u32,
}

impl Name {
    pub fn new(prefix: &str) -> Name {
        Name {
            prefix: prefix.to_string(),
            counter: 0,
        }
    }

    pub fn gen(&mut self) -> String {
        let tmp = self.counter;
        self.counter += 1;
        format!("{}{}", self.prefix, tmp)
    }
}

impl From<asm::Prog> for Module {
    fn from(prog: asm::Prog) -> Self {
        let mut m = Module::new_with_name(&prog.id());
        for input in prog.inputs().iter() {
            m.add_input(&input.id(), input.ty().width());
        }
        for output in prog.outputs().iter() {
            m.add_output(&output.id(), output.ty().width());
        }
        let mut name = Name::new("i");
        for instr in prog.body().iter() {
            let mut inst = Instance::from(instr.clone());
            inst.set_id(&name.gen());
            m.add_instance(inst);
        }
        m
    }
}
