use std::collections::HashMap;
use verilog::ast as vl;

pub type ConnectionMap = HashMap<String, vl::Expr>;
pub type WidthMap = HashMap<String, u32>;

#[derive(Clone, Debug, Default)]
pub struct Port {
    pub width: WidthMap,
    pub connection: ConnectionMap,
}

pub trait DefaultPort {
    // implement default inputs
    fn default_input_port() -> Port;
    // implement default outputs
    fn default_output_port() -> Port;
}

impl Port {
    pub fn get_width(&self, port: &str) -> Option<&u32> {
        self.width.get(port)
    }
    pub fn width(&self) -> &WidthMap {
        &self.width
    }
    pub fn connection(&self) -> &ConnectionMap {
        &self.connection
    }
}
