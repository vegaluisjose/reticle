use crate::ir::ast as ir;
use crate::ml::ast as ml;
use crate::verilog::ast as verilog;
use std::collections::HashSet;

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

impl From<ir::ExprTup> for Vec<verilog::Id> {
    fn from(tup: ir::ExprTup) -> Self {
        let mut ids: Vec<verilog::Id> = Vec::new();
        for t in tup.term() {
            let i: Vec<verilog::Id> = t.clone().into();
            ids.extend(i);
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

impl From<ir::Expr> for Vec<verilog::Id> {
    fn from(expr: ir::Expr) -> Self {
        match &expr {
            ir::Expr::Tup(tup) => tup.clone().into(),
            ir::Expr::Term(term) => term.clone().into(),
        }
    }
}

impl From<ir::Expr> for Vec<verilog::Decl> {
    fn from(expr: ir::Expr) -> Self {
        match &expr {
            ir::Expr::Tup(tup) => tup.clone().into(),
            ir::Expr::Term(term) => term.clone().into(),
        }
    }
}

impl From<ir::InstrWire> for Vec<verilog::Decl> {
    fn from(instr: ir::InstrWire) -> Self {
        instr.dst().clone().into()
    }
}

impl From<ir::Instr> for Vec<verilog::Decl> {
    fn from(instr: ir::Instr) -> Self {
        match &instr {
            ir::Instr::Wire(instr) => instr.clone().into(),
            _ => unimplemented!(),
        }
    }
}

impl From<ir::Sig> for verilog::Module {
    fn from(sig: ir::Sig) -> Self {
        let id = sig.id();
        let mut module = verilog::Module::new(&id);
        let input: Vec<verilog::Decl> = sig.input().clone().into();
        let output: Vec<verilog::Decl> = sig.output().clone().into();
        for i in input {
            module.add_port(verilog::Port::Input(i.clone()))
        }
        for o in output {
            module.add_port(verilog::Port::Output(o.clone()))
        }
        module
    }
}

impl From<ir::Def> for verilog::Module {
    fn from(def: ir::Def) -> Self {
        let mut module = verilog::Module::from(def.sig().clone());
        let output: Vec<verilog::Decl> = def.sig().output().clone().into();
        let output_set: HashSet<verilog::Decl> = output.into_iter().collect();
        let mut instr: Vec<verilog::Decl> = Vec::new();
        for i in def.body() {
            let d: Vec<verilog::Decl> = i.clone().into();
            instr.extend(d);
        }
        let instr_set: HashSet<verilog::Decl> = instr.into_iter().collect();
        for d in instr_set.difference(&output_set) {
            module.add_decl(d.clone());
        }
        module
    }
}

impl From<ml::InstrBasc> for Vec<verilog::Decl> {
    fn from(instr: ml::InstrBasc) -> Self {
        instr.dst().clone().into()
    }
}

impl From<ml::InstrMach> for Vec<verilog::Decl> {
    fn from(instr: ml::InstrMach) -> Self {
        instr.dst().clone().into()
    }
}

impl From<ml::Instr> for Vec<verilog::Decl> {
    fn from(instr: ml::Instr) -> Self {
        match &instr {
            ml::Instr::Basc(instr) => instr.clone().into(),
            ml::Instr::Mach(instr) => instr.clone().into(),
        }
    }
}

impl From<ml::OpMach> for verilog::Id {
    fn from(op: ml::OpMach) -> Self {
        match op {
            ml::OpMach::Lut1 => "LUT1".to_string(),
            ml::OpMach::Lut2 => "LUT2".to_string(),
            ml::OpMach::Lut3 => "LUT3".to_string(),
            ml::OpMach::Lut4 => "LUT4".to_string(),
            ml::OpMach::Lut5 => "LUT5".to_string(),
            ml::OpMach::Lut6 => "LUT6".to_string(),
            ml::OpMach::Fdre => "FDRE".to_string(),
            ml::OpMach::Fdse => "FDSE".to_string(),
            ml::OpMach::Dsp => "DSP48E2".to_string(),
            ml::OpMach::Carry => "CARRY8".to_string(),
        }
    }
}
