use crate::lang::ast::*;
use std::str::FromStr;

impl Port {
    pub fn id(&self) -> Id {
        match self {
            Port::Input { id, ty: _ } => id.to_string(),
            Port::Output { id, ty: _ } => id.to_string(),
        }
    }

    pub fn ty(&self) -> &DataType {
        match self {
            Port::Input { id: _, ty } => ty,
            Port::Output { id: _, ty } => ty,
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

impl Instr {
    pub fn new_placed(dst: &str, ty: &str, op: &str, lhs: &str, rhs: &str) -> Instr {
        Instr::Placed {
            id: dst.to_string(),
            ty: DataType::from_str(ty).unwrap(),
            op: PlacedOp::from_str(op).unwrap(),
            attrs: vec![],
            params: vec![Expr::Ref(lhs.to_string()), Expr::Ref(rhs.to_string())],
            loc: Loc::Unknown,
        }
    }

    pub fn id(&self) -> String {
        match self {
            Instr::Std {
                id,
                ty: _,
                op: _,
                attrs:_,
                params: _,
            } => id.to_string(),
            Instr::Placed {
                id,
                ty: _,
                op: _,
                attrs: _,
                params: _,
                loc: _,
            } => id.to_string(),
        }
    }

    pub fn ty(&self) -> &DataType {
        match self {
            Instr::Std {
                id:_,
                ty,
                op: _,
                attrs:_,
                params: _,
            } => ty,
            Instr::Placed {
                id:_,
                ty,
                op: _,
                attrs: _,
                params: _,
                loc: _,
            } => ty,
        }
    }

    pub fn params(&self) -> &Vec<Expr> {
        match self {
            Instr::Std {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params,
            } => params,
            Instr::Placed {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params,
                loc: _,
            } => params,
        }
    }

    pub fn attrs(&self) -> &Vec<Expr> {
        match self {
            Instr::Std {
                id: _,
                ty: _,
                op: _,
                attrs,
                params: _,
            } => attrs,
            Instr::Placed {
                id: _,
                ty: _,
                op: _,
                attrs,
                params: _,
                loc: _,
            } => attrs,
        }
    }

    pub fn placed_op(&self) -> &PlacedOp {
        match self {
            Instr::Placed {
                id: _,
                ty: _,
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
            Instr::Placed {
                id: _,
                ty: _,
                op: _,
                attrs: _,
                params: _,
                loc,
            } => loc,
            _ => panic!("Error: std ops don't support location"),
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

    pub fn body(&self) -> &Vec<Instr> {
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
        let ty = DataType::from_str(ty).unwrap();
        let port = Port::Input {
            id: name.to_string(),
            ty: ty,
        };
        self.inputs.push(port);
    }

    pub fn add_output(&mut self, name: &str, ty: &str) {
        let ty = DataType::from_str(ty).unwrap();
        let port = Port::Output {
            id: name.to_string(),
            ty: ty,
        };
        self.outputs.push(port);
    }

    pub fn add_instr(&mut self, instr: Instr) {
        self.body.push(instr);
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
