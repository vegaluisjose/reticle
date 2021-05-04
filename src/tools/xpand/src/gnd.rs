use crate::port::Output;
use verilog::ast as vl;

#[derive(Clone, Debug)]
pub struct Gnd {
    pub name: String,
    pub prim: String,
    pub output: Output,
}

impl Default for Gnd {
    fn default() -> Self {
        let gnd = "gnd";
        let name = format!("_{}", gnd);
        Gnd {
            name,
            prim: "GND".to_string(),
            output: Output::gnd(gnd),
        }
    }
}

impl Gnd {
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn to_instance(&self) -> vl::Instance {
        let mut inst = vl::Instance::new(&self.name, &self.prim);
        for (k, v) in self.output.connection.iter() {
            inst.connect(&k, v.clone());
        }
        inst
    }
    pub fn to_stmt(&self) -> vl::Stmt {
        vl::Stmt::from(self.to_instance())
    }
}
