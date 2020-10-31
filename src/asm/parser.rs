use crate::asm::ast::*;
// use crate::lang::check::Check;
// use crate::lang::infer::infer_expr_types;
use crate::util::file::read_to_string;
use pest_consume::{match_nodes, Error, Parser};
use std::path::Path;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

const _GRAMMAR: &str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "asm/grammar.pest"]
struct AsmParser;

#[pest_consume::parser]
impl AsmParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    fn id(input: Node) -> Result<Id> {
        Ok(input.as_str().to_string())
    }

    fn num(input: Node) -> Result<i64> {
        Ok(input.as_str().parse::<i64>().unwrap())
    }

    fn ty(input: Node) -> Result<Ty> {
        Ok(Ty::from_str(input.as_str()).unwrap())
    }

    fn expr(input: Node) -> Result<Expr> {
        Ok(match_nodes!(
            input.into_children();
            [id(n), ty(ty)] => Expr::new_ref(&n, ty),
            [id(n)] => Expr::new_ref(&n, Ty::Hole),
            [num(n)] => Expr::new_int(n),
        ))
    }

    fn inputs(input: Node) -> Result<Vec<Port>> {
        Ok(match_nodes!(
            input.into_children();
            [expr(expr)..] => expr.into_iter().map(|e| Port::Input(e)).collect()
        ))
    }

    fn outputs(input: Node) -> Result<Vec<Port>> {
        Ok(match_nodes!(
            input.into_children();
            [expr(expr)..] => expr.into_iter().map(|e| Port::Output(e)).collect()
        ))
    }

    fn instr_phy(input: Node) -> Result<InstrPrim> {
        Ok(match_nodes!(
            input.into_children();
            [expr(dst), id(op)] => InstrPrim {
                op,
                dst,
                attrs: vec![],
                params: vec![],
                loc: Loc {
                    prim: LocTy::Hole,
                    x: ExprCoord::Hole,
                    y: ExprCoord::Hole,
                }
            }
        ))
    }

    fn instr(input: Node) -> Result<Instr> {
        Ok(match_nodes!(
            input.into_children();
            [instr_phy(instr)] => Instr::Prim(instr)
        ))
    }

    fn instrs(input: Node) -> Result<Vec<Instr>> {
        Ok(match_nodes!(
            input.into_children();
            [instr(instr)..] => instr.collect()
        ))
    }

    fn prog(input: Node) -> Result<Prog> {
        Ok(match_nodes!(
            input.into_children();
            [id(id), inputs(inputs), outputs(outputs), instrs(body)] => Prog {
                sig: Signature {
                    id,
                    inputs,
                    outputs,
                },
                body,
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

pub fn parse(input_str: &str) -> Prog {
    let inputs = AsmParser::parse(Rule::file, input_str).expect("Error: parsing inputs");
    let input = inputs.single().expect("Error: parsing root");
    AsmParser::file(input).expect("Error: parsing file")
    // let prog_infer = infer_expr_types(&prog);
    // prog_infer.check();
    // prog_infer
}

pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Prog {
    let content = read_to_string(path);
    parse(&content)
}
