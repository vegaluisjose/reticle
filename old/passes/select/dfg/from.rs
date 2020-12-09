use crate::lang::ast::{Instr, Port, Prog};
use crate::passes::select::dfg::{Dfg, DfgNodeValue};

impl From<Port> for DfgNodeValue {
    fn from(port: Port) -> Self {
        match port {
            Port::Input { .. } => DfgNodeValue::Inp(port),
            Port::Output { .. } => DfgNodeValue::Out(port),
        }
    }
}

impl From<Instr> for DfgNodeValue {
    fn from(instr: Instr) -> Self {
        DfgNodeValue::Ins(instr)
    }
}

impl From<Prog> for Dfg {
    fn from(prog: Prog) -> Self {
        let mut dfg = Dfg::default();
        if let Some(def) = prog.defs().iter().next() {
            for input in def.inputs().iter() {
                dfg.add_node(&input.id(), DfgNodeValue::from(input.clone()));
            }
            for instr in def.body().iter() {
                dfg.add_node(&instr.dst_id(), DfgNodeValue::from(instr.clone()));
            }
            for instr in def.body().iter() {
                for param in instr.params().iter() {
                    dfg.add_edge(&param.id(), &instr.dst_id());
                }
            }
        }
        dfg.find_roots(&prog);
        dfg
    }
}
