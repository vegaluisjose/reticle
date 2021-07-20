use crate::ast::*;
use crate::errors::Error;
use crate::infer::infer_type_try_from_target;
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

    fn cost(input: Node) -> ParseResult<u64> {
        let val = input.as_str().parse::<u64>();
        match val {
            Ok(v) => Ok(v),
            Err(_) => panic!("Error: parsing {} as dec u64", input.as_str()),
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

    fn bel_block(input: Node) -> ParseResult<Bel> {
        let bel = BelBlock::from_str(input.as_str());
        match bel {
            Ok(t) => Ok(t.into()),
            Err(m) => panic!("{}", m),
        }
    }

    fn bel_dsp(input: Node) -> ParseResult<Bel> {
        let bel = BelDsp::from_str(input.as_str());
        match bel {
            Ok(t) => Ok(t.into()),
            Err(m) => panic!("{}", m),
        }
    }

    fn bel_reg(input: Node) -> ParseResult<Bel> {
        let bel = BelReg::from_str(input.as_str());
        match bel {
            Ok(t) => Ok(t.into()),
            Err(m) => panic!("{}", m),
        }
    }

    fn bel_carry(input: Node) -> ParseResult<Bel> {
        let bel = BelCarry::from_str(input.as_str());
        match bel {
            Ok(t) => Ok(t.into()),
            Err(m) => panic!("{}", m),
        }
    }

    fn bel_lut(input: Node) -> ParseResult<Bel> {
        let bel = BelLut::from_str(input.as_str());
        match bel {
            Ok(t) => Ok(t.into()),
            Err(m) => panic!("{}", m),
        }
    }

    fn bel_lum(input: Node) -> ParseResult<Bel> {
        let bel = BelLum::from_str(input.as_str());
        match bel {
            Ok(t) => Ok(t.into()),
            Err(m) => panic!("{}", m),
        }
    }

    fn loc_lut(input: Node) -> ParseResult<Loc> {
        Ok(match_nodes!(
            input.into_children();
            [bel_lut(bel), expr_coord(x), expr_coord(y)] => Loc {
                bel,
                x,
                y,
            },
        ))
    }

    fn loc_lum(input: Node) -> ParseResult<Loc> {
        Ok(match_nodes!(
            input.into_children();
            [bel_lum(bel), expr_coord(x), expr_coord(y)] => Loc {
                bel,
                x,
                y,
            },
        ))
    }

    fn loc_block(input: Node) -> ParseResult<Loc> {
        Ok(match_nodes!(
            input.into_children();
            [bel_block(bel), expr_coord(x), expr_coord(y)] => Loc {
                bel,
                x,
                y,
            },
        ))
    }

    fn loc_dsp(input: Node) -> ParseResult<Loc> {
        Ok(match_nodes!(
            input.into_children();
            [bel_dsp(bel), expr_coord(x), expr_coord(y)] => Loc {
                bel,
                x,
                y,
            },
        ))
    }

    fn loc_reg(input: Node) -> ParseResult<Loc> {
        Ok(match_nodes!(
            input.into_children();
            [bel_reg(bel), expr_coord(x), expr_coord(y)] => Loc {
                bel,
                x,
                y,
            },
        ))
    }

    fn loc_carry(input: Node) -> ParseResult<Loc> {
        Ok(match_nodes!(
            input.into_children();
            [bel_carry(bel), expr_coord(x), expr_coord(y)] => Loc {
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

    fn op_block(input: Node) -> ParseResult<OpMach> {
        let op = OpMach::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn op_dsp(input: Node) -> ParseResult<OpMach> {
        let op = OpMach::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn op_reg(input: Node) -> ParseResult<OpMach> {
        let op = OpMach::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn op_carry(input: Node) -> ParseResult<OpMach> {
        let op = OpMach::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn op_lut(input: Node) -> ParseResult<OpMach> {
        let op = OpMach::from_str(input.as_str());
        match op {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn op_lum(input: Node) -> ParseResult<OpMach> {
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

    fn instr_block(input: Node) -> ParseResult<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_block(op), io(arg)] => Instr::from(InstrMach {
                op,
                attr: Expr::default(),
                dst,
                arg,
                loc: None,
                mem: None,
            }),
            [io(dst), op_block(op), io(arg), loc_block(loc)] => Instr::from(InstrMach {
                op,
                attr: Expr::default(),
                dst,
                arg,
                loc: Some(loc),
                mem: None,
            })
        ))
    }

    fn instr_dsp(input: Node) -> ParseResult<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_dsp(op), io(arg)] => Instr::from(InstrMach {
                op,
                attr: Expr::default(),
                dst,
                arg,
                loc: None,
                mem: None,
            }),
            [io(dst), op_dsp(op), io(arg), loc_dsp(loc)] => Instr::from(InstrMach {
                op,
                attr: Expr::default(),
                dst,
                arg,
                loc: Some(loc),
                mem: None,
            })
        ))
    }

    fn instr_reg(input: Node) -> ParseResult<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_reg(op), io(arg)] => Instr::from(InstrMach {
                op,
                attr: Expr::default(),
                dst,
                arg,
                loc: None,
                mem: None,
            }),
            [io(dst), op_reg(op), io(arg), loc_reg(loc)] => Instr::from(InstrMach {
                op,
                attr: Expr::default(),
                dst,
                arg,
                loc: Some(loc),
                mem: None,
            })
        ))
    }

    fn instr_carry(input: Node) -> ParseResult<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_carry(op), io(arg)] => Instr::from(InstrMach {
                op,
                attr: Expr::default(),
                dst,
                arg,
                loc: None,
                mem: None,
            }),
            [io(dst), op_carry(op), io(arg), loc_carry(loc)] => Instr::from(InstrMach {
                op,
                attr: Expr::default(),
                dst,
                arg,
                loc: Some(loc),
                mem: None,
            })
        ))
    }

    fn instr_lut(input: Node) -> ParseResult<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_lut(op), io(arg)] => Instr::from(InstrMach {
                op,
                attr: Expr::default(),
                dst,
                arg,
                loc: None,
                mem: None,
            }),
            [io(dst), op_lut(op), io(arg), loc_lut(loc)] => Instr::from(InstrMach {
                op,
                attr: Expr::default(),
                dst,
                arg,
                loc: Some(loc),
                mem: None,
            }),
            [io(dst), op_lut(op), tup_val(attr), io(arg)] => Instr::from(InstrMach {
                op,
                attr: Expr::from(attr),
                dst,
                arg,
                loc: None,
                mem: None,
            }),
            [io(dst), op_lut(op), tup_val(attr), io(arg), loc_lut(loc)] => Instr::from(InstrMach {
                op,
                attr: Expr::from(attr),
                dst,
                arg,
                loc: Some(loc),
                mem: None,
            })
        ))
    }

    fn instr_lum(input: Node) -> ParseResult<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_lum(op), io(arg)] => Instr::from(InstrMach {
                op,
                attr: Expr::default(),
                dst,
                arg,
                loc: None,
                mem: None,
            }),
            [io(dst), op_lum(op), io(arg), loc_lum(loc)] => Instr::from(InstrMach {
                op,
                attr: Expr::default(),
                dst,
                arg,
                loc: Some(loc),
                mem: None,
            })
        ))
    }

    fn instr_basc(input: Node) -> ParseResult<Instr> {
        Ok(match_nodes!(
            input.into_children();
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

    fn instr(input: Node) -> ParseResult<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [instr_basc(instr)] => instr,
            [instr_reg(instr)] => instr,
            [instr_carry(instr)] => instr,
            [instr_dsp(instr)] => instr,
            [instr_block(instr)] => instr,
            [instr_lut(instr)] => instr,
            [instr_lum(instr)] => instr,
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
            [id(id), cost(area), cost(perf)] => Sig {
                id,
                input: Expr::default(),
                output: Expr::default(),
                area,
                perf,
            },
            [id(id), cost(area), cost(perf), io(output)] => Sig {
                id,
                input: Expr::default(),
                output,
                area,
                perf,
            },
            [id(id), cost(area), cost(perf), io(input), io(output)] => Sig {
                id,
                input,
                output,
                area,
                perf,
            },
        ))
    }

    fn imp(input: Node) -> ParseResult<Imp> {
        Ok(match_nodes!(
            input.into_children();
            [sig(sig)] => Imp {
                sig,
                body: Vec::new(),
            },
            [sig(sig), body(body)] => Imp {
                sig,
                body,
            },
        ))
    }

    fn target(input: Node) -> ParseResult<Target> {
        Ok(match_nodes!(
            input.into_children();
            [imp(imp)..] => {
                let mut target = Target::default();
                let imps: Vec<Imp> = imp.collect();
                for i in imps {
                    target.insert(&i.id(), i.clone());
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

impl Parser {
    pub fn parse_from_str(input_str: &str) -> Result<Target, Error> {
        let inputs = Parser::parse(Rule::file, input_str)?;
        let input = inputs.single()?;
        let target = Parser::file(input)?;
        Ok(infer_type_try_from_target(&target))
    }
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<Target, Error> {
        let content = read_to_string(path);
        Parser::parse_from_str(&content)
    }
}
