use crate::tdl::ast::*;
use crate::util::errors::Error;
use crate::util::file::read_to_string;
use pest_consume::Error as PestError;
use pest_consume::{match_nodes, Parser};
use std::path::Path;
use std::str::FromStr;

pub type ParseResult<T> = std::result::Result<T, PestError<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const _GRAMMAR: &str = include_str!("syntax.pest");

#[derive(Parser)]
#[grammar = "tdl/syntax.pest"]
pub struct TDLParser;

#[pest_consume::parser]
impl TDLParser {
    fn EOI(_input: Node) -> ParseResult<()> {
        Ok(())
    }

    fn id(input: Node) -> ParseResult<Id> {
        Ok(input.as_str().to_string())
    }

    fn val(input: Node) -> ParseResult<ExprTerm> {
        let val = input.as_str().parse::<i64>();
        match val {
            Ok(v) => Ok(ExprTerm::Val(v)),
            Err(_) => panic!("Error: parsing {} as i64", input.as_str()),
        }
    }

    fn cost(input: Node) -> ParseResult<u64> {
        let val = input.as_str().parse::<u64>();
        match val {
            Ok(v) => Ok(v),
            Err(_) => panic!("Error: parsing {} as u64", input.as_str()),
        }
    }

    fn ty(input: Node) -> ParseResult<Ty> {
        let ty = Ty::from_str(input.as_str());
        match ty {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn prim(input: Node) -> ParseResult<Prim> {
        let prim = Prim::from_str(input.as_str());
        match prim {
            Ok(p) => Ok(p),
            Err(m) => panic!("{}", m),
        }
    }

    fn var(input: Node) -> ParseResult<ExprTerm> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), ty(ty)] => ExprTerm::Var(id, ty),
            [id(id)] => ExprTerm::Var(id, Ty::Any),
        ))
    }

    fn tup_var(input: Node) -> ParseResult<ExprTup> {
        Ok(match_nodes!(
            input.into_children();
            [var(vars)..] => ExprTup{ term: vars.collect() },
        ))
    }

    fn tup_val(input: Node) -> ParseResult<ExprTup> {
        Ok(match_nodes!(
            input.into_children();
            [val(vals)..] => ExprTup{ term: vals.collect() },
        ))
    }

    fn io(input: Node) -> ParseResult<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [var(var)] => Expr::from(var),
            [tup_var(tup)] => Expr::from(tup),
        ))
    }

    fn op_comp(input: Node) -> ParseResult<OpComp> {
        let op = OpComp::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn op_wire(input: Node) -> ParseResult<OpWire> {
        let op = OpWire::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn op_mach(input: Node) -> ParseResult<OpMach> {
        let op = OpMach::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn op_basc(input: Node) -> ParseResult<OpBasc> {
        let op = OpBasc::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn pat_instr(input: Node) -> ParseResult<PatInstr> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_comp(op), io(arg)] => PatInstr::from(InstrComp {
                op,
                dst,
                attr: Expr::default(),
                arg,
                prim: Prim::Any,
            }),
            [io(dst), op_comp(op), tup_val(attr), io(arg)] => PatInstr::from(InstrComp {
                op,
                dst,
                attr: Expr::from(attr),
                arg,
                prim: Prim::Any,
            }),
            [io(dst), op_comp(op), io(arg), prim(prim)] => PatInstr::from(InstrComp {
                op,
                dst,
                attr: Expr::default(),
                arg,
                prim,
            }),
            [io(dst), op_comp(op), tup_val(attr), io(arg), prim(prim)] => PatInstr::from(InstrComp {
                op,
                dst,
                attr: Expr::from(attr),
                arg,
                prim,
            }),
            [io(dst), op_wire(op), tup_val(attr)] => PatInstr::from(InstrWire {
                op,
                dst,
                attr: Expr::from(attr),
                arg: Expr::default(),
            }),
            [io(dst), op_wire(op), io(arg)] => PatInstr::from(InstrWire {
                op,
                dst,
                attr: Expr::default(),
                arg,
            }),
            [io(dst), op_wire(op), tup_val(attr), io(arg)] => PatInstr::from(InstrWire {
                op,
                dst,
                attr: Expr::from(attr),
                arg,
            }),
        ))
    }

    fn pat_body(input: Node) -> ParseResult<Vec<PatInstr>> {
        Ok(match_nodes!(
            input.into_children();
            [pat_instr(instr)..] => instr.collect(),
        ))
    }

    fn pat_sig(input: Node) -> ParseResult<PatSig> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), prim(prim), cost(area), cost(lat), io(input), io(output)] => PatSig {
                id,
                prim,
                area,
                lat,
                input,
                output,
            },
        ))
    }

    fn imp_sig(input: Node) -> ParseResult<ImpSig> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), io(input), io(output)] => ImpSig {
                id,
                x: ExprCoord::Any,
                y: ExprCoord::Any,
                input,
                output,
            },
        ))
    }

    fn pat(input: Node) -> ParseResult<Pat> {
        Ok(match_nodes!(
            input.into_children();
            [pat_sig(sig), pat_body(body)] => Pat {
                sig,
                body,
            },
        ))
    }

    fn imp(input: Node) -> ParseResult<Imp> {
        Ok(match_nodes!(
            input.into_children();
            [imp_sig(sig)] => Imp {
                sig,
                body: Vec::new(),
            },
        ))
    }

    fn des(input: Node) -> ParseResult<Des> {
        Ok(match_nodes!(
            input.into_children();
            [imp(imp)] => Des::from(imp),
            [pat(pat)] => Des::from(pat),
        ))
    }

    fn body(input: Node) -> ParseResult<Vec<Des>> {
        Ok(match_nodes!(
            input.into_children();
            [des(des)..] => des.collect(),
        ))
    }

    fn target(input: Node) -> ParseResult<Target> {
        Ok(match_nodes!(
            input.into_children();
            [body(body)] => {
                let mut target = Target::default();
                for d in body {
                    match d {
                        Des::Imp(imp) => target.add_imp(&imp.id(), imp.clone()),
                        Des::Pat(pat) => target.add_pat(&pat.id(), pat.clone()),
                    }
                }
                target
            }
        ))
    }

    fn file(input: Node) -> ParseResult<Target> {
        Ok(match_nodes!(
            input.into_children();
            [target(target), _] => target,
        ))
    }
}

impl TDLParser {
    pub fn parse_from_str(input_str: &str) -> Result<Target, Error> {
        let inputs = TDLParser::parse(Rule::file, input_str)?;
        let input = inputs.single()?;
        Ok(TDLParser::file(input)?)
    }
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<Target, Error> {
        let content = read_to_string(path);
        TDLParser::parse_from_str(&content)
    }
}
