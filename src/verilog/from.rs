use crate::ir::ast as ir;
use crate::verilog::ast as verilog;

impl From<ir::ExprTerm> for Vec<verilog::Id> {
    fn from(term: ir::ExprTerm) -> Self {
        let mut ids: Vec<verilog::Id> = Vec::new();
        if let Some(ty) = term.ty() {
            if let Some(id) = term.id() {
                if let Some(length) = ty.length() {
                    for i in 0..length {
                        ids.push(format!("{}_{}", id, i));
                    }
                } else {
                    ids.push(id);
                }
            }
        }
        ids
    }
}

impl From<ir::ExprTerm> for Vec<verilog::Decl> {
    fn from(term: ir::ExprTerm) -> Self {
        let ids: Vec<verilog::Id> = term.clone().into();
        let mut decls: Vec<verilog::Decl> = Vec::new();
        if let Some(ty) = term.ty() {
            if let Some(width) = ty.width() {
                for id in ids {
                    let wire = verilog::Decl::new_wire(&id, width);
                    decls.push(wire);
                }
            }
        }
        decls
    }
}

impl From<ir::ExprTup> for Vec<verilog::Decl> {
    fn from(tup: ir::ExprTup) -> Self {
        let mut decls: Vec<verilog::Decl> = Vec::new();
        for term in tup.term() {
            let t: Vec<verilog::Decl> = term.clone().into();
            decls.extend(t);
        }
        decls
    }
}

impl From<ir::Expr> for Vec<verilog::Decl> {
    fn from(expr: ir::Expr) -> Self {
        let decls: Vec<verilog::Decl> = match expr {
            ir::Expr::Tup(tup) => tup.clone().into(),
            ir::Expr::Term(term) => term.clone().into(),
        };
        decls
    }
}
