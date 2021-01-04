use crate::util::errors::Error;
use crate::util::file::read_to_string;
use crate::xl::ast::*;
use pest_consume::Error as PestError;
use pest_consume::{match_nodes, Parser};
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;

pub type ParseResult<T> = std::result::Result<T, PestError<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const _GRAMMAR: &str = include_str!("syntax.pest");

#[derive(Parser)]
#[grammar = "xl/syntax.pest"]
pub struct XLParser;

#[pest_consume::parser]
impl XLParser {
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

    fn ty(input: Node) -> ParseResult<Ty> {
        let ty = Ty::from_str(input.as_str());
        match ty {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn op_coord(input: Node) -> ParseResult<OpCoord> {
        let op = OpCoord::from_str(input.as_str());
        match op {
            Ok(e) => Ok(e),
            Err(m) => panic!("{}", m),
        }
    }

    fn coord(input: Node) -> ParseResult<ExprCoord> {
        let expr = ExprCoord::from_str(input.as_str());
        match expr {
            Ok(e) => Ok(e),
            Err(m) => panic!("{}", m),
        }
    }

    fn expr_coord(input: Node) -> ParseResult<ExprCoord> {
        Ok(match_nodes!(
            input.into_children();
            [coord(coord)] => coord,
            [coord(lhs), op_coord(op), coord(rhs)] => ExprCoord::Bin(op, Rc::new(lhs), Rc::new(rhs)),
        ))
    }

    fn bel(input: Node) -> ParseResult<Bel> {
        let bel = Bel::from_str(input.as_str());
        match bel {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn loc(input: Node) -> ParseResult<Loc> {
        Ok(match_nodes!(
            input.into_children();
            [bel(bel), expr_coord(x), expr_coord(y)] => Loc {
                bel,
                x,
                y,
            },
        ))
    }

    fn var(input: Node) -> ParseResult<ExprTerm> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), ty(ty)] => ExprTerm::Var(id, ty),
            [id(id)] => ExprTerm::Var(id, Ty::Any),
            [] => ExprTerm::Any,
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

    fn opt_key(input: Node) -> ParseResult<Opt> {
        let opt = Opt::from_str(input.as_str());
        match opt {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn opt_num(input: Node) -> ParseResult<OptVal> {
        let val = input.as_str().parse::<u64>();
        match val {
            Ok(v) => Ok(OptVal::UInt(v)),
            Err(_) => panic!("Error: parsing {} as u64", input.as_str()),
        }
    }

    fn opt_op(input: Node) -> ParseResult<OptVal> {
        let op = OpDsp::from_str(input.as_str());
        match op {
            Ok(v) => Ok(OptVal::Op(v)),
            Err(_) => panic!("Error: parsing {} as u64", input.as_str()),
        }
    }

    fn opt_tup(input: Node) -> ParseResult<(Opt, OptVal)> {
        Ok(match_nodes!(
            input.into_children();
            [opt_key(key), opt_num(val)] => (key, val),
            [opt_key(key), opt_op(val)] => (key, val)
        ))
    }

    fn opt(input: Node) -> ParseResult<OptMap> {
        Ok(match_nodes!(
            input.into_children();
            [opt_tup(tup)..] => {
                let mut map = OptMap::new();
                for (key, val) in tup {
                    map.insert(key, val);
                }
                map
            }
        ))
    }

    fn instr_mach(input: Node) -> ParseResult<InstrMach> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_mach(op), io(arg)] => InstrMach {
                op,
                opt: OptMap::new(),
                dst,
                arg,
                loc: None,
            },
            [io(dst), op_mach(op), opt(opt), io(arg)] => InstrMach {
                op,
                opt,
                dst,
                arg,
                loc: None,
            },
            [io(dst), op_mach(op), io(arg), loc(loc)] => InstrMach {
                op,
                opt: OptMap::new(),
                dst,
                arg,
                loc: Some(loc),
            },
            [io(dst), op_mach(op), opt(opt), io(arg), loc(loc)] => InstrMach {
                op,
                opt,
                dst,
                arg,
                loc: Some(loc),
            }
        ))
    }

    fn instr_basc(input: Node) -> ParseResult<InstrBasc> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_basc(op), tup_val(attr)] => InstrBasc {
                op,
                dst,
                attr: Expr::from(attr),
                arg: Expr::default(),
            },
            [io(dst), op_basc(op), io(arg)] => InstrBasc {
                op,
                dst,
                attr: Expr::default(),
                arg,
            },
            [io(dst), op_basc(op), tup_val(attr), io(arg)] => InstrBasc {
                op,
                dst,
                attr: Expr::from(attr),
                arg,
            }
        ))
    }

    fn instr(input: Node) -> ParseResult<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [instr_mach(instr)] => Instr::from(instr),
            [instr_basc(instr)] => Instr::from(instr),
        ))
    }

    fn body(input: Node) -> ParseResult<Vec<Instr>> {
        Ok(match_nodes!(
            input.into_children();
            [instr(instr)..] => instr.collect(),
        ))
    }

    fn sig(input: Node) -> ParseResult<Sig> {
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

    fn prog(input: Node) -> ParseResult<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [sig(sig), body(body)] => Prog {
                sig,
                body,
            },
        ))
    }

    fn file(input: Node) -> ParseResult<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [prog(prog), _] => prog,
        ))
    }
}

impl XLParser {
    pub fn parse_from_str(input_str: &str) -> Result<Prog, Error> {
        let inputs = XLParser::parse(Rule::file, input_str)?;
        let input = inputs.single()?;
        Ok(XLParser::file(input)?)
    }
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<Prog, Error> {
        let content = read_to_string(path);
        XLParser::parse_from_str(&content)
    }
}
