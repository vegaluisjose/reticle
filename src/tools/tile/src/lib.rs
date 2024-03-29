use asm::ast::{Expr, ExprTerm, ExprTup, Instr, InstrAsm, InstrWire, Loc, OpAsm, OpWire, Prog, Ty};
use asm::parser::Parser;
use regex::Regex;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Namer {
    pub prefix: String,
    pub cnt: u64,
}

impl Namer {
    pub fn new(init: u64) -> Self {
        Namer {
            prefix: "t".to_string(),
            cnt: init,
        }
    }
    pub fn next_name(&mut self) -> String {
        let name = format!("{}{}", self.prefix, self.cnt);
        self.cnt += 1;
        name
    }
}

fn parse_number(input: &str) -> i64 {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"^t(\d+)$").unwrap();
    }
    match RE.captures(input) {
        Some(caps) => {
            if let Some(val) = caps.get(1) {
                if let Ok(num) = val.as_str().parse::<i64>() {
                    num
                } else {
                    -1
                }
            } else {
                -1
            }
        }
        _ => -1,
    }
}

fn get_max_expr(expr: &Expr) -> i64 {
    let mut val: Vec<i64> = Vec::new();
    let exprs: Vec<ExprTerm> = expr.clone().into();
    for e in exprs {
        if let Some(id) = e.id() {
            val.push(parse_number(&id));
        }
    }
    let mut max = -1;
    for v in val {
        if v > max {
            max = v;
        }
    }
    max
}

fn get_max_instr(instr: &Instr) -> i64 {
    let a = get_max_expr(instr.dst());
    let b = get_max_expr(instr.arg());
    if a > b {
        a
    } else {
        b
    }
}

fn get_max(body: &[Instr]) -> i64 {
    let mut val = -1;
    for i in body {
        let cur = get_max_instr(i);
        if cur > val {
            val = cur;
        }
    }
    val
}

fn emit_term(name: &str, width: u64) -> ExprTerm {
    ExprTerm::Var(name.to_string(), Ty::SInt(width))
}

fn emit_term_bool(name: &str) -> ExprTerm {
    ExprTerm::Var(name.to_string(), Ty::Bool)
}

fn emit_expr(name: &str, width: u64) -> Expr {
    Expr::Term(emit_term(name, width))
}

fn emit_ext(instr: &InstrAsm, dst: Expr, arg: usize, start: i64, end: i64) -> Instr {
    let attr_term = vec![ExprTerm::Val(start), ExprTerm::Val(end)];
    let attr = Expr::from(ExprTup { term: attr_term });
    let arg_term = vec![instr.arg().get_term(arg).unwrap().clone()];
    let arg = Expr::from(ExprTup { term: arg_term });
    Instr::from(InstrWire {
        op: OpWire::Ext,
        dst,
        attr,
        arg,
    })
}

fn emit_cat(dst: Expr, arg: &[String]) -> Instr {
    let mut arg_term: Vec<ExprTerm> = Vec::new();
    for a in arg {
        arg_term.push(emit_term(a, 8));
    }
    Instr::from(InstrWire {
        op: OpWire::Cat,
        dst,
        attr: Expr::default(),
        arg: Expr::from(ExprTup::from(arg_term)),
    })
}

fn emit_op(op: &str, dst: &str, arg: &[String], loc: &Loc) -> Instr {
    let op_asm: OpAsm = op.to_string().into();
    let dst = emit_expr(dst, 8);
    let mut arg_term: Vec<ExprTerm> = Vec::new();
    for a in arg {
        arg_term.push(emit_term(a, 8));
    }
    Instr::from(InstrAsm {
        op: op_asm,
        dst,
        arg: Expr::from(ExprTup::from(arg_term)),
        loc: loc.clone(),
    })
}

fn emit_op_mux(op: &str, dst: &str, arg: &[String], loc: &Loc) -> Instr {
    let op_asm: OpAsm = op.to_string().into();
    let dst = emit_expr(dst, 8);
    let mut arg_term: Vec<ExprTerm> = Vec::new();
    for (i, a) in arg.iter().enumerate() {
        if i == 1 || i == 2 {
            arg_term.push(emit_term(a, 8));
        } else {
            arg_term.push(emit_term_bool(a));
        }
    }
    Instr::from(InstrAsm {
        op: op_asm,
        dst,
        arg: Expr::from(ExprTup::from(arg_term)),
        loc: loc.clone(),
    })
}

pub fn tile_from_prog(input: &Prog) -> Prog {
    let mut body: Vec<Instr> = Vec::new();
    let max = get_max(input.body());
    let init = if max < 0 { 0_u64 } else { (max + 1) as u64 };
    let mut namer = Namer::new(init);
    for instr in input.body() {
        match instr {
            Instr::Wire(_) => body.push(instr.clone()),
            Instr::Asm(asm) => {
                match asm.op().to_string().as_str() {
                    "lxor_i128" | "lxor_i32" => {
                        let num = (asm.dst().get_ty(0).unwrap().width().unwrap() / 8) as i64;
                        let mut cat: Vec<String> = Vec::new();
                        for i in 0..num {
                            let mut arg: Vec<String> = Vec::new();
                            for a in 0..2 {
                                let name = namer.next_name();
                                arg.push(name.clone());
                                let dst = emit_expr(&name, 8);
                                let new = emit_ext(asm, dst, a, i * 8, (i + 1) * 8 - 1);
                                body.push(new);
                            }
                            let name = namer.next_name();
                            cat.push(name.clone());
                            let new = emit_op("lxor_i8", &name, &arg, asm.loc());
                            body.push(new);
                        }
                        let new = emit_cat(asm.dst().clone(), &cat);
                        body.push(new);
                    }
                    "lmux_i128" => {
                        let num = (asm.dst().get_ty(0).unwrap().width().unwrap() / 8) as i64;
                        let mut cat: Vec<String> = Vec::new();
                        for i in 0..num {
                            let c = asm.arg().get_id(0).unwrap();
                            let mut arg: Vec<String> = Vec::new();
                            arg.push(c);
                            for a in 1..3 {
                                let name = namer.next_name();
                                arg.push(name.clone());
                                let dst = emit_expr(&name, 8);
                                let new = emit_ext(asm, dst, a, i * 8, (i + 1) * 8 - 1);
                                body.push(new);
                            }
                            let name = namer.next_name();
                            cat.push(name.clone());
                            let new = emit_op_mux("lmux_i8", &name, &arg, asm.loc());
                            body.push(new);
                        }
                        let new = emit_cat(asm.dst().clone(), &cat);
                        body.push(new);
                    }
                    "lmuxrega_i128" => {
                        let num = (asm.dst().get_ty(0).unwrap().width().unwrap() / 8) as i64;
                        let mut cat: Vec<String> = Vec::new();
                        for i in 0..num {
                            let c = asm.arg().get_id(0).unwrap();
                            let mut arg: Vec<String> = Vec::new();
                            arg.push(c);
                            for a in 1..3 {
                                let name = namer.next_name();
                                arg.push(name.clone());
                                let dst = emit_expr(&name, 8);
                                let new = emit_ext(asm, dst, a, i * 8, (i + 1) * 8 - 1);
                                body.push(new);
                            }
                            let e = asm.arg().get_id(3).unwrap();
                            arg.push(e);
                            let name = namer.next_name();
                            cat.push(name.clone());
                            let new = emit_op_mux("lmuxrega_i8", &name, &arg, asm.loc());
                            body.push(new);
                        }
                        let new = emit_cat(asm.dst().clone(), &cat);
                        body.push(new);
                    }
                    _ => body.push(instr.clone()),
                };
            }
        }
    }
    let mut prog = Prog::default();
    prog.set_sig(input.sig().clone());
    prog.set_body(body);
    prog
}

pub fn tile_from_file<P: AsRef<Path>>(path: P) -> Prog {
    let parsed = Parser::parse_from_file(path).unwrap();
    tile_from_prog(&parsed)
}
