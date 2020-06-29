use crate::lang::ast;
use std::fmt;
use std::rc::Rc;

pub struct DAG {
    name: String,
}

impl DAG {
    pub fn new() -> DAG {
        DAG {
            name: String::new(),
        }
    }

    pub fn from_ast(&self, prog: &ast::Prog) {
        assert!(prog.defs.len() == 1, "single def only supported atm");
        let comp = match &prog.defs[0] {
            ast::Def::Comp {
                name: _,
                inputs: _,
                outputs: _,
                body,
            } => body.clone(),
            _ => panic!("Error Sim component not supported"),
        };
        println!("Hello from DAG {}", self.name);
        for decl in comp.iter() {
            println!("{}", decl);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    op: ast::PlacedOp,
    params: Vec<Node>,
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
            params: Vec::new(),
            ty: ty.clone(),
            cost: cost,
            loc: loc.clone(),
        }
    }

    pub fn change_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn push_param(&mut self, op: &Node) {
        self.params.push(op.clone());
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name:{} cost:{} loc:{}", self.name, self.cost, self.loc)
    }
}

pub fn pattern_example() {
    let mut patterns: Vec<Node> = Vec::new();
    let mut dsp_add = Node::new_with_attrs(
        &ast::PlacedOp::Add,
        &ast::DataType::UInt(8),
        4,
        &ast::Expr::Origin(
            ast::PlacedType::Dsp,
            Rc::new(ast::Expr::Placeholder),
            Rc::new(ast::Expr::Placeholder),
        ),
    );
    let mut dsp_mul = Node::new_with_attrs(
        &ast::PlacedOp::Mul,
        &ast::DataType::UInt(8),
        4,
        &ast::Expr::Origin(
            ast::PlacedType::Dsp,
            Rc::new(ast::Expr::Placeholder),
            Rc::new(ast::Expr::Placeholder),
        ),
    );
    let mut dsp_reg_mul = Node::new_with_attrs(
        &ast::PlacedOp::Reg,
        &ast::DataType::UInt(8),
        3,
        &ast::Expr::Origin(
            ast::PlacedType::Dsp,
            Rc::new(ast::Expr::Placeholder),
            Rc::new(ast::Expr::Placeholder),
        ),
    );
    dsp_reg_mul.push_param(&dsp_mul);
    dsp_add.change_name("add");
    dsp_mul.change_name("mul");
    dsp_reg_mul.change_name("reg_mul");
    patterns.push(dsp_add);
    patterns.push(dsp_mul);
    patterns.push(dsp_reg_mul);
    for pat in patterns.iter() {
        println!("{}", pat);
    }
}
