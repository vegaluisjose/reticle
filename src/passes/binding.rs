use crate::lang::ast;
use std::fmt;
use std::rc::Rc;
use std::collections::HashMap;

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

pub fn print_patterns() {
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
    dsp_r_add.push_param(&dsp_add);
    lut_r_add.push_param(&lut_add);
    lut_add.change_name("add");
    lut_r_add.change_name("r_add");
    dsp_add.change_name("add");
    dsp_r_add.change_name("r_add");
    patterns.push(lut_add);
    patterns.push(lut_r_add);
    patterns.push(dsp_add);
    patterns.push(dsp_r_add);
    println!("\n-----patterns-----");
    for pat in patterns.iter() {
        println!("{}", pat);
    }
    println!("\n");
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

    pub fn from_ast(&self, prog: &ast::Prog) {
        assert!(prog.defs.len() == 1, "single def only supported atm");
        let (comp_def, comp_outputs) = match &prog.defs[0] {
            ast::Def::Comp {
                name: _,
                inputs: _,
                outputs,
                body,
            } => (body.clone(), outputs.clone()),
            _ => panic!("Error Sim component not supported"),
        };
        for decl in comp_def.iter() {
            match decl {
                ast::Decl::Comp { op, outputs } => {
                    assert!(outputs.len() == 1, "Error single output for now");
                    match &outputs[0] {
                        ast::Port::Output {id, ..} => {
                            println!("id: {}", id);
                        },
                        _ => panic!("Error: should not be input"),
                    }
                },
                _ => panic!("Error: Sim decl not supported"),
            }
        }
    }
}
