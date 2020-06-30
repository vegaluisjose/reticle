use crate::lang::ast;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    op: ast::PlacedOp,
    inputs: Vec<Node>,
    ty: ast::DataType,
    cost: u128,
    loc: ast::Expr,
}

impl Node {
    pub fn new_with_attrs(
        op: &ast::PlacedOp,
        ty: &ast::DataType,
        cost: u128,
        loc: &ast::Expr,
    ) -> Node {
        Node {
            name: String::new(),
            op: op.clone(),
            inputs: Vec::new(),
            ty: ty.clone(),
            cost: cost,
            loc: loc.clone(),
        }
    }

    pub fn from_compop(cop: &ast::CompOp, cty: &ast::DataType) -> Node {
        match cop {
            ast::CompOp::Placed {
                op,
                attrs: _,
                inputs: _,
                loc,
            } => Node::new_with_attrs(op, cty, 100, loc),
            _ => panic!("Error: std ops do not have cost"),
        }
    }

    pub fn change_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn push_input(&mut self, op: &Node) {
        self.inputs.push(op.clone());
    }

    pub fn has_placeholder(&self) -> bool {
        match self.loc {
            ast::Expr::Placeholder => true,
            _ => false,
        }
    }

    pub fn postorder(&self) -> Vec<Node> {
        let mut stack: Vec<Node> = Vec::new();
        let mut res: Vec<Node> = Vec::new();
        stack.push(self.clone());
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            res.push(node.clone());
            for operand in node.inputs.iter() {
                stack.push(operand.clone());
            }
        }
        res.reverse();
        res
    }

    pub fn estimate(&self) -> u128 {
        let mut sum = self.cost;
        for i in self.inputs.iter() {
            sum += i.estimate();
        }
        sum
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        if self.op != other.op {
            false
        } else if self.ty != other.ty {
            false
        } else if self.inputs.len() != other.inputs.len() {
            false
        } else {
            let mut eq = true;
            for (a, b) in self.inputs.iter().zip(other.inputs.iter()) {
                if a.op != b.op {
                    eq = false;
                    break;
                } else if a.ty != b.ty {
                    eq = false;
                    break;
                } else if a.inputs.len() != b.inputs.len() {
                    eq = false;
                    break;
                }
            }
            eq
        }
    }
}

impl Eq for Node {}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name:{} cost:{} loc:{}", self.name, self.cost, self.loc)
    }
}

pub fn get_patterns() -> Vec<Node> {
    let mut patterns: Vec<Node> = Vec::new();
    let mut dsp_add = Node::new_with_attrs(
        &ast::PlacedOp::Add,
        &ast::DataType::UInt(8),
        3,
        &ast::Expr::Origin(
            ast::PlacedType::Dsp,
            Rc::new(ast::Expr::Placeholder),
            Rc::new(ast::Expr::Placeholder),
        ),
    );
    let mut lut_add = Node::new_with_attrs(
        &ast::PlacedOp::Add,
        &ast::DataType::UInt(8),
        4,
        &ast::Expr::Origin(
            ast::PlacedType::Lut,
            Rc::new(ast::Expr::Placeholder),
            Rc::new(ast::Expr::Placeholder),
        ),
    );
    let mut dsp_r_add = Node::new_with_attrs(
        &ast::PlacedOp::Reg,
        &ast::DataType::UInt(8),
        1,
        &ast::Expr::Origin(
            ast::PlacedType::Dsp,
            Rc::new(ast::Expr::Placeholder),
            Rc::new(ast::Expr::Placeholder),
        ),
    );
    let mut lut_r_add = Node::new_with_attrs(
        &ast::PlacedOp::Reg,
        &ast::DataType::UInt(8),
        1,
        &ast::Expr::Origin(
            ast::PlacedType::Lut,
            Rc::new(ast::Expr::Placeholder),
            Rc::new(ast::Expr::Placeholder),
        ),
    );
    dsp_r_add.push_input(&dsp_add);
    lut_r_add.push_input(&lut_add);
    lut_add.change_name("add");
    lut_r_add.change_name("r_add");
    dsp_add.change_name("add");
    dsp_r_add.change_name("r_add");
    patterns.push(lut_add);
    patterns.push(lut_r_add);
    patterns.push(dsp_add);
    patterns.push(dsp_r_add);
    patterns
}

pub struct DAG {
    root: Option<Node>,
    env: HashMap<ast::Id, Node>,
}

impl DAG {
    pub fn new() -> DAG {
        DAG {
            root: None,
            env: HashMap::new(),
        }
    }

    pub fn from_ast(&mut self, prog: &ast::Prog) {
        assert!(prog.defs.len() == 1, "single def only supported atm");
        let body = match &prog.defs[0] {
            ast::Def::Comp {
                name: _,
                inputs: _,
                outputs: _,
                body,
            } => body.clone(),
            _ => panic!("Error Sim component not supported"),
        };
        for decl in body.iter() {
            match decl {
                ast::Decl::Comp { op, outputs } => {
                    assert!(outputs.len() == 1, "Error: single output for now");
                    match &outputs[0] {
                        ast::Port::Output { id, datatype } => {
                            if !self.env.contains_key(id) {
                                let mut node = Node::from_compop(op, datatype);
                                for i in op.get_inputs().iter() {
                                    let name = i.clone().get_id();
                                    if self.env.contains_key(&name) {
                                        node.push_input(self.env.get(&name).unwrap());
                                    }
                                }
                                self.env.insert(id.to_string(), node.clone());
                            }
                            self.root = Some(self.env.get(id).unwrap().clone());
                        }
                        _ => panic!("Error: should not be input"),
                    }
                }
                _ => panic!("Error: sim decl not supported"),
            }
        }
    }

    pub fn get_root(self) -> Node {
        match self.root {
            Some(node) => node,
            _ => panic!("Error: node has not been built yet"),
        }
    }
}

pub fn opt(code: &Node, patterns: &Vec<Node>) -> Vec<Node> {
    let mut codegen: Vec<Node> = Vec::new();
    for instr in code.postorder().iter() {
        let mut best = u128::max_value();
        let mut found = false;
        for pat in patterns.iter() {
            if instr == pat {
                if best == u128::max_value() {
                    best = instr.estimate();
                }
                let cost = pat.estimate();
                if cost < best && instr.has_placeholder() {
                    if found {
                        codegen.pop().unwrap();
                    }
                    codegen.push(pat.clone());
                    found = true;
                    best = cost;
                }
            }
        }
        if !found {
            codegen.push(instr.clone());
        }
    }
    codegen
}
