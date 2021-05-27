use verilog::ast as vl;

pub trait ToExpr {
    fn to_expr(&self) -> vl::Expr;
}
