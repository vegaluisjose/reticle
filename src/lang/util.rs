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
            _ => panic!("Error: debug decl don't support outputs"),
        }
    }

    pub fn params(&self) -> &Vec<Expr> {
        match self {
            Decl::Instr { op, outputs: _ } => op.params(),
            _ => panic!("Error: debug decl don't support params"),
        }
    }

    pub fn loc(&self) -> &Loc {
        match self {
            Decl::Instr { op, outputs: _ } => op.loc(),
            _ => panic!("Error: debug decl don't support loc"),
        }
    }

    pub fn placed_op(&self) -> &PlacedOp {
        match self {
            Decl::Instr { op, outputs: _ } => op.placed_op(),
            _ => panic!("Error: debug decl don't support placed op"),
        }
    }
}

impl Def {
    pub fn new_comp(name: &str) -> Def {
        Def::Comp {
            name: name.to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn body(&self) -> &Vec<Decl> {
        match self {
            Def::Comp {
                name: _,
                inputs: _,
                outputs: _,
                body,
            } => body,
            Def::Sim { name: _, body } => body,
        }
    }

    pub fn inputs(&self) -> &Vec<Port> {
        match self {
            Def::Comp {
                name: _,
                inputs,
                outputs: _,
                body: _,
            } => inputs,
            _ => panic!("Error: sim definition don't support inputs")
        }
    }

    pub fn outputs(&self) -> &Vec<Port> {
        match self {
            Def::Comp {
                name: _,
                inputs: _,
                outputs,
                body: _,
            } => outputs,
            _ => panic!("Error: sim definition don't support outputs")
        }
    }

    pub fn add_input(&mut self, name: &str, ty: &str) {
        match self {
            Def::Comp {
                name: _,
                inputs,
                outputs: _,
                body: _,
            } => {
                let dtype = DataType::from_str(ty).unwrap();
                let port = Port::Input {
                    id: name.to_string(),
                    datatype: dtype,
                };
                inputs.push(port);
            }
            _ => panic!("Error: sim definition does not support inputs"),
        }
    }

    pub fn add_output(&mut self, name: &str, ty: &str) {
        match self {
            Def::Comp {
                name: _,
                inputs: _,
                outputs,
                body: _,
            } => {
                let dtype = DataType::from_str(ty).unwrap();
                let port = Port::Output {
                    id: name.to_string(),
                    datatype: dtype,
                };
                outputs.push(port);
            }
            _ => panic!("Error: sim definition does not support outputs"),
        }
    }

    pub fn add_decl(&mut self, decl: Decl) {
        match self {
            Def::Comp {
                name: _,
                inputs: _,
                outputs: _,
                body,
            } => {
                body.push(decl);
            }
            Def::Sim { name: _, body } => {
                body.push(decl);
            }
        }
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
