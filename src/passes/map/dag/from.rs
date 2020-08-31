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
                dag.add_node(&input.id(), DagNodeValue::from(input.clone()));
            }
            for instr in def.body().iter() {
                dag.add_node(&instr.dst_id(), DagNodeValue::from(instr.clone()));
            }
            for instr in def.body().iter() {
                for param in instr.params().iter() {
                    dag.add_edge(&param.id(), &instr.dst_id());
                }
            }
        }
        dag.find_roots(&prog);
        dag
    }
}
