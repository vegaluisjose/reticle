use crate::ml::ast::*;
use crate::util::file::read_to_string;
use pest_consume::{match_nodes, Error, Parser};
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const _GRAMMAR: &str = include_str!("syntax.pest");

#[derive(Parser)]
#[grammar = "ml/syntax.pest"]
pub struct MLParser;

#[pest_consume::parser]
impl MLParser {
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

    fn op_coord(input: Node) -> Result<OpCoord> {
        let op = OpCoord::from_str(input.as_str());
        match op {
            Ok(e) => Ok(e),
            Err(m) => panic!("{}", m),
        }
    }

    fn coord(input: Node) -> Result<ExprCoord> {
        let expr = ExprCoord::from_str(input.as_str());
        match expr {
            Ok(e) => Ok(e),
            Err(m) => panic!("{}", m),
        }
    }

    fn expr_coord(input: Node) -> Result<ExprCoord> {
        Ok(match_nodes!(
            input.into_children();
            [coord(coord)] => coord,
            [coord(lhs), op_coord(op), coord(rhs)] => ExprCoord::Bin(op, Rc::new(lhs), Rc::new(rhs)),
        ))
    }

    fn bel(input: Node) -> Result<Bel> {
        let bel = Bel::from_str(input.as_str());
        match bel {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn loc(input: Node) -> Result<Loc> {
        Ok(match_nodes!(
            input.into_children();
            [bel(bel), expr_coord(x), expr_coord(y)] => Loc {
                bel: Some(bel),
                x,
                y,
            },
            [expr_coord(x), expr_coord(y)] => Loc {
                bel: None,
                x,
                y,
            },
        ))
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

    fn op_mach(input: Node) -> Result<OpMach> {
        let op = OpMach::from_str(input.as_str());
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

    fn instr_mach(input: Node) -> Result<InstrMach> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_mach(op), io(arg), loc(loc)] => InstrMach {
                op,
                opt: OptMap::new(),
                dst,
                arg,
                loc
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

    fn instr(input: Node) -> Result<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [instr_mach(instr)] => Instr::from(instr),
            [instr_wire(instr)] => Instr::from(instr),
        ))
    }

    fn file(input: Node) -> Result<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [instr(instr), _] => instr,
        ))
    }
}

impl MLParser {
    pub fn parse_from_str(input_str: &str) -> Result<Instr> {
        let inputs = MLParser::parse(Rule::file, input_str)?;
        let input = inputs.single()?;
        Ok(MLParser::file(input)?)
    }
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<Instr> {
        let content = read_to_string(path);
        MLParser::parse_from_str(&content)
    }
}
