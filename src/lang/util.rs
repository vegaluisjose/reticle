use crate::lang::ast::*;
use std::str::FromStr;

impl Port {
    pub fn id(&self) -> Id {
        match self {
            Port::Input { id, datatype: _ } => id.to_string(),
            Port::Output { id, datatype: _ } => id.to_string(),
        }
    }

    pub fn datatype(&self) -> &DataType {
        match self {
            Port::Input { id: _, datatype } => datatype,
            Port::Output { id: _, datatype } => datatype,
        }
    }
}

impl Expr {
    pub fn id(&self) -> Id {
        match self {
            Expr::Ref(name) => name.to_string(),
            _ => panic!("Error: expr is not Ref"),
        }
    }
}

impl Op {
    pub fn params(&self) -> &Vec<Expr> {
        match self {
            Op::Std {
                op: _,
                attrs: _,
                params,
            } => params,
            Op::Placed {
                op: _,
                attrs: _,
                params,
                loc: _,
            } => params,
        }
    }

    pub fn attrs(&self) -> &Vec<Expr> {
        match self {
            Op::Std {
                op: _,
                attrs,
                params: _,
            } => attrs,
            Op::Placed {
                op: _,
                attrs,
                params: _,
                loc: _,
            } => attrs,
        }
    }

    pub fn placed_op(&self) -> &PlacedOp {
        match self {
            Op::Placed {
                op,
                attrs: _,
                params: _,
                loc: _,
            } => op,
            _ => panic!("Error: std ops don't support placed op"),
        }
    }

    pub fn loc(&self) -> &Loc {
        match self {
            Op::Placed {
                op: _,
                attrs: _,
                params: _,
                loc,
            } => loc,
            _ => panic!("Error: std ops don't support location"),
        }
    }
}

impl Decl {
    pub fn new_instr(dst: &str, ty: &str, op: &str, lhs: &str, rhs: &str) -> Decl {
        let placed_op = Op::Placed {
            op: PlacedOp::from_str(op).unwrap(),
            attrs: vec![],
            params: vec![Expr::Ref(lhs.to_string()), Expr::Ref(rhs.to_string())],
            loc: Loc::Unknown,
        };
        let output = Port::Output {
            id: dst.to_string(),
            datatype: DataType::from_str(ty).unwrap(),
        };
        Decl::Instr {
            op: placed_op,
            outputs: vec![output],
        }
    }

    pub fn outputs(&self) -> &Vec<Port> {
        match self {
            Decl::Instr { op: _, outputs } => outputs,
        }
    }

    pub fn params(&self) -> &Vec<Expr> {
        match self {
            Decl::Instr { op, outputs: _ } => op.params(),
        }
    }

    pub fn loc(&self) -> &Loc {
        match self {
            Decl::Instr { op, outputs: _ } => op.loc(),
        }
    }

    pub fn placed_op(&self) -> &PlacedOp {
        match self {
            Decl::Instr { op, outputs: _ } => op.placed_op(),
        }
    }
}

impl Def {
    pub fn new_comp(name: &str) -> Def {
        Def {
            name: name.to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn new_with_ports(name: &str, inputs: &Vec<Port>, outputs: &Vec<Port>) -> Def {
        Def {
            name: name.to_string(),
            inputs: inputs.to_vec(),
            outputs: outputs.to_vec(),
            body: Vec::new(),
        }
    }

    pub fn body(&self) -> &Vec<Decl> {
        &self.body
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn inputs(&self) -> &Vec<Port> {
        &self.inputs
    }

    pub fn outputs(&self) -> &Vec<Port> {
        &self.outputs
    }

    pub fn add_input(&mut self, name: &str, ty: &str) {
        let dtype = DataType::from_str(ty).unwrap();
        let port = Port::Input {
            id: name.to_string(),
            datatype: dtype,
        };
        self.inputs.push(port);
    }

    pub fn add_output(&mut self, name: &str, ty: &str) {
        let dtype = DataType::from_str(ty).unwrap();
        let port = Port::Output {
            id: name.to_string(),
            datatype: dtype,
        };
        self.outputs.push(port);
    }

    pub fn add_decl(&mut self, decl: Decl) {
        self.body.push(decl);
    }
}

impl Prog {
    pub fn new() -> Prog {
        Prog { defs: Vec::new() }
    }

    pub fn add_def(&mut self, def: Def) {
        self.defs.push(def);
    }
}
