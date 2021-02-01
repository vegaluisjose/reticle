use crate::tdl::ast::*;
use crate::tdl::infer::{try_infer_imp, try_infer_pat, try_infer_prim};
use crate::util::errors::Error;
use crate::util::file::read_to_string;
use pest_consume::Error as PestError;
use pest_consume::{match_nodes, Parser};
use std::path::Path;
use std::rc::Rc;
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

    fn opt_key(input: Node) -> ParseResult<Opt> {
        let opt = Opt::from_str(input.as_str());
        match opt {
            Ok(t) => Ok(t),
            Err(m) => panic!("{}", m),
        }
    }

    fn dec_num(input: Node) -> ParseResult<OptVal> {
        let val = input.as_str().parse::<u64>();
        match val {
            Ok(v) => Ok(OptVal::UInt(v)),
            Err(_) => panic!("Error: parsing {} as dec u64", input.as_str()),
        }
    }

    fn bin_num(input: Node) -> ParseResult<OptVal> {
        let val = u64::from_str_radix(input.as_str(), 2);
        match val {
            Ok(v) => Ok(OptVal::UInt(v)),
            Err(_) => panic!("Error: parsing {} as bin u64", input.as_str()),
        }
    }

    fn hex_num(input: Node) -> ParseResult<OptVal> {
        let val = u64::from_str_radix(input.as_str(), 16);
        match val {
            Ok(v) => Ok(OptVal::UInt(v)),
            Err(_) => panic!("Error: parsing {} as hex u64", input.as_str()),
        }
    }

    fn opt_num(input: Node) -> ParseResult<OptVal> {
        Ok(match_nodes!(
            input.into_children();
            [dec_num(num)] => num,
            [hex_num(num)] => num,
            [bin_num(num)] => num,
        ))
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

    fn pat(input: Node) -> ParseResult<Pat> {
        Ok(match_nodes!(
            input.into_children();
            [pat_sig(sig), pat_body(body)] => Pat {
                sig,
                body,
            },
        ))
    }

    fn imp_instr(input: Node) -> ParseResult<ImpInstr> {
        Ok(match_nodes!(
            input.into_children();
            [io(dst), op_mach(op), io(arg)] => ImpInstr::from(InstrMach {
                op,
                opt: OptMap::new(),
                dst,
                arg,
                loc: None,
            }),
            [io(dst), op_mach(op), opt(opt), io(arg)] => ImpInstr::from(InstrMach {
                op,
                opt,
                dst,
                arg,
                loc: None,
            }),
            [io(dst), op_mach(op), io(arg), loc(loc)] => ImpInstr::from(InstrMach {
                op,
                opt: OptMap::new(),
                dst,
                arg,
                loc: Some(loc),
            }),
            [io(dst), op_mach(op), opt(opt), io(arg), loc(loc)] => ImpInstr::from(InstrMach {
                op,
                opt,
                dst,
                arg,
                loc: Some(loc),
            }),
            [io(dst), op_basc(op), tup_val(attr)] => ImpInstr::from(InstrBasc {
                op,
                dst,
                attr: Expr::from(attr),
                arg: Expr::default(),
            }),
            [io(dst), op_basc(op), io(arg)] => ImpInstr::from(InstrBasc {
                op,
                dst,
                attr: Expr::default(),
                arg,
            }),
            [io(dst), op_basc(op), tup_val(attr), io(arg)] => ImpInstr::from(InstrBasc {
                op,
                dst,
                attr: Expr::from(attr),
                arg,
            })
        ))
    }

    fn imp_body(input: Node) -> ParseResult<Vec<ImpInstr>> {
        Ok(match_nodes!(
            input.into_children();
            [imp_instr(instr)..] => instr.collect(),
        ))
    }

    fn imp_sig(input: Node) -> ParseResult<ImpSig> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), coord(x), coord(y), io(input), io(output)] => ImpSig {
                id,
                x,
                y,
                input,
                output,
            },
        ))
    }

    fn imp(input: Node) -> ParseResult<Imp> {
        Ok(match_nodes!(
            input.into_children();
            [imp_sig(sig), imp_body(body)] => Imp {
                sig,
                body,
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
        let mut target = TDLParser::file(input)?;
        let pat = target.pat().clone();
        for (n, p) in pat {
            let typed_pat= try_infer_pat(&p)?;
            target.add_pat(&n, try_infer_prim(&typed_pat));
        }
        let imp = target.imp().clone();
        for (n, i) in imp {
            target.add_imp(&n, try_infer_imp(&i)?);
        }
        Ok(target)
    }
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<Target, Error> {
        let content = read_to_string(path);
        TDLParser::parse_from_str(&content)
    }
}
