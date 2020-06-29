use crate::lang::ast;

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
            ast::Def::Comp { name:_, inputs:_, outputs:_, body } => body.clone(),
            _ => panic!("Error Sim component not supported"),
        };
        println!("Hello from DAG {}", self.name);
        for decl in comp.iter() {
            println!("{}", decl);
        }
    }
}
