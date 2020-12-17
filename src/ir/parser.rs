use crate::ir::ast::*;
use crate::util::file::read_to_string;
use pest_consume::{match_nodes, Error, Parser};
use std::path::Path;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const _GRAMMAR: &str = include_str!("syntax.pest");

#[derive(Parser)]
#[grammar = "ir/syntax.pest"]
pub struct IRParser;

#[pest_consume::parser]
impl IRParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    fn id(input: Node) -> Result<Id> {
        Ok(input.as_str().to_string())
    }

    fn val(input: Node) -> Result<Expr> {
        let val = input.as_str().parse::<i64>();
        match val {
            Ok(v) => Ok(Expr::Val(v)),
            Err(_) => panic!("Error: parsing {} as i64", input.as_str()),
        }
    }

    fn ty(input: Node) -> Result<Ty> {
        let ty = Ty::from_str(input.as_str());
        match ty {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn prim(input: Node) -> Result<Prim> {
        let prim = Prim::from_str(input.as_str());
        match prim {
            Ok(p) => Ok(p),
            Err(m) => panic!("{}", m),
        }
    }

    fn var(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), ty(ty)] => Expr::Var(id, ty),
            [id(id)] => Expr::Var(id, Ty::Any),
        ))
    }

    fn tup_var(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [var(vars)..] => Expr::from(ExprTup{ expr: vars.collect()}),
        ))
    }

    fn tup_val(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [val(vals)..] => Expr::from(ExprTup{ expr: vals.collect()}),
        ))
    }

    fn io(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [var(var)] => var,
            [tup_var(tup)] => tup,
        ))
    }

    fn op_comp(input: Node) -> Result<OpComp> {
        let op = OpComp::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn op_wire(input: Node) -> Result<OpWire> {
        let op = OpWire::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn op_call(input: Node) -> Result<OpCall> {
        let op = OpCall::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn instr_comp(input: Node) -> Result<InstrComp> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_comp(op), io(arg)] => InstrComp {
                op,
                dst,
                attr: Expr::default(),
                arg,
                prim: Prim::Any,
            },
            [io(dst), op_comp(op), tup_val(attr), io(arg)] => InstrComp {
                op,
                dst,
                attr,
                arg,
                prim: Prim::Any,
            },
            [io(dst), op_comp(op), io(arg), prim(prim)] => InstrComp {
                op,
                dst,
                attr: Expr::default(),
                arg,
                prim,
            },
            [io(dst), op_comp(op), tup_val(attr), io(arg), prim(prim)] => InstrComp {
                op,
                dst,
                attr,
                arg,
                prim,
            }
        ))
    }

    fn instr_wire(input: Node) -> Result<InstrWire> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_wire(op), tup_val(attr)] => InstrWire {
                op,
                dst,
                attr,
                arg: Expr::default(),
            },
            [io(dst), op_wire(op), io(arg)] => InstrWire {
                op,
                dst,
                attr: Expr::default(),
                arg,
            },
            [io(dst), op_wire(op), tup_val(attr), io(arg)] => InstrWire {
                op,
                dst,
                attr,
                arg,
            }
        ))
    }

    fn instr_call(input: Node) -> Result<InstrCall> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_call(op)] => InstrCall {
                op,
                dst,
                arg: Expr::default(),
            },
            [io(dst), op_call(op), io(arg)] => InstrCall {
                op,
                dst,
                arg,
            },
        ))
    }

    fn instr(input: Node) -> Result<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [instr_comp(instr)] => Instr::from(instr),
            [instr_wire(instr)] => Instr::from(instr),
            [instr_call(instr)] => Instr::from(instr),
        ))
    }

    fn body(input: Node) -> Result<Vec<Instr>> {
        Ok(match_nodes!(
            input.into_children();
            [instr(instr)..] => instr.collect(),
        ))
    }

    fn sig(input: Node) -> Result<Sig> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), io(output)] => Sig {
                id,
                input: Expr::default(),
                output,
            },
            [id(id), io(input), io(output)] => Sig {
                id,
                input,
                output,
            },
        ))
    }

    fn def(input: Node) -> Result<Def> {
        Ok(match_nodes!(
            input.into_children();
            [sig(sig), body(body)] => Def {
                sig,
                body,
            },
        ))
    }

    fn prog(input: Node) -> Result<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [def(def)..] => {
                let mut prog = Prog::default();
                let defs: Vec<Def> = def.collect();
                for d in defs {
                    prog.insert(&d.id(), d.clone());
                }
                prog
            }
        ))
    }

    fn file(input: Node) -> Result<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [prog(prog), _] => prog,
        ))
    }
}

impl IRParser {
    pub fn parse_from_str(input_str: &str) -> Result<Prog> {
        let inputs = IRParser::parse(Rule::file, input_str)?;
        let input = inputs.single()?;
        Ok(IRParser::file(input)?)
    }
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<Prog> {
        let content = read_to_string(path);
        IRParser::parse_from_str(&content)
    }
}
