use crate::ast::*;
use crate::errors::Error;
use crate::infer::infer_type_try_from_prog;
use io::read_to_string;
use pest_consume::match_nodes;
use pest_consume::Error as PestError;
use pest_consume::Parser as PestParser;
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;

pub type ParseResult<T> = std::result::Result<T, PestError<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const _GRAMMAR: &str = include_str!("syntax.pest");

#[derive(PestParser)]
#[grammar = "syntax.pest"]
pub struct Parser;

#[pest_consume::parser]
impl Parser {
    fn EOI(_input: Node) -> ParseResult<()> {
        Ok(())
    }

    fn id(input: Node) -> ParseResult<Id> {
        Ok(input.as_str().to_string())
    }

    fn val_bin(input: Node) -> ParseResult<ExprTerm> {
        let val = u64::from_str_radix(input.as_str(), 2);
        match val {
            Ok(v) => Ok(ExprTerm::Val(v as i64)),
            Err(_) => panic!("Error: parsing {} as bin i64", input.as_str()),
        }
    }

    fn val_hex(input: Node) -> ParseResult<ExprTerm> {
        let val = u64::from_str_radix(input.as_str(), 16);
        match val {
            Ok(v) => Ok(ExprTerm::Val(v as i64)),
            Err(_) => panic!("Error: parsing {} as hex i64", input.as_str()),
        }
    }

    fn val_dec(input: Node) -> ParseResult<ExprTerm> {
        let val = input.as_str().parse::<i64>();
        match val {
            Ok(v) => Ok(ExprTerm::Val(v)),
            Err(_) => panic!("Error: parsing {} as dec i64", input.as_str()),
        }
    }

    fn val(input: Node) -> ParseResult<ExprTerm> {
        Ok(match_nodes!(
            input.into_children();
            [val_bin(bin)] => bin,
            [val_hex(hex)] => hex,
            [val_dec(dec)] => dec,
        ))
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

    fn instr(input: Node) -> ParseResult<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_mach(op), io(arg)] => Instr::from(InstrMach {
                op,
                attr: Expr::default(),
                dst,
                arg,
                loc: None,
            }),
            [io(dst), op_mach(op), io(arg), loc(loc)] => Instr::from(InstrMach {
                op,
                attr: Expr::default(),
                dst,
                arg,
                loc: Some(loc),
            }),
            [io(dst), op_mach(op), tup_val(attr), io(arg)] => Instr::from(InstrMach {
                op,
                attr: Expr::from(attr),
                dst,
                arg,
                loc: None,
            }),
            [io(dst), op_mach(op), tup_val(attr), io(arg), loc(loc)] => Instr::from(InstrMach {
                op,
                attr: Expr::from(attr),
                dst,
                arg,
                loc: Some(loc),
            }),
            [io(dst), op_basc(op), tup_val(attr)] => Instr::from(InstrBasc {
                op,
                dst,
                attr: Expr::from(attr),
                arg: Expr::default(),
            }),
            [io(dst), op_basc(op), io(arg)] => Instr::from(InstrBasc {
                op,
                dst,
                attr: Expr::default(),
                arg,
            }),
            [io(dst), op_basc(op), tup_val(attr), io(arg)] => Instr::from(InstrBasc {
                op,
                dst,
                attr: Expr::from(attr),
                arg,
            })
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
            [id(id)] => Sig {
                id,
                input: Expr::default(),
                output: Expr::default(),
            },
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
            [sig(sig)] => Prog {
                sig,
                body: Vec::new(),
            },
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

impl Parser {
    pub fn parse_from_str(input_str: &str) -> Result<Prog, Error> {
        let inputs = Parser::parse(Rule::file, input_str)?;
        let input = inputs.single()?;
        let prog = Parser::file(input)?;
        Ok(infer_type_try_from_prog(&prog))
    }
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<Prog, Error> {
        let content = read_to_string(path);
        Parser::parse_from_str(&content)
    }
}
