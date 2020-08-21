use crate::lang::ast::{Instr, Port, Prog};
use crate::passes::map::dag::{Dag, DagNodeValue};

impl From<Port> for DagNodeValue {
    fn from(port: Port) -> Self {
        match port {
            Port::Input { .. } => DagNodeValue::Inp(port),
            Port::Output { .. } => DagNodeValue::Out(port),
        }
    }
}

impl From<Instr> for DagNodeValue {
    fn from(instr: Instr) -> Self {
        DagNodeValue::Ins(instr)
    }
}

impl From<Prog> for Dag {
    fn from(prog: Prog) -> Self {
        let mut dag = Dag::default();
        if let Some(def) = prog.defs().iter().next() {
            for input in def.inputs().iter() {
                if !dag.contains_node_with_id(&input.id()) {
                    let val = DagNodeValue::from(input.clone());
                    dag.add_node(&input.id(), val);
                }
            }
            for instr in def.body().iter() {
                if !dag.contains_node_with_id(&instr.id()) {
                    let val = DagNodeValue::from(instr.clone());
                    dag.add_node(&instr.id(), val);
                }
            }
            for instr in def.body().iter() {
                for param in instr.params().iter() {
                    dag.add_edge(&param.id(), &instr.id());
                }
            }
        }
        dag.find_roots(&prog);
        dag
    }
}
