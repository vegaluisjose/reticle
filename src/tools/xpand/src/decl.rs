use verilog::ast as vl;

pub trait ToDecl {
    fn to_decl(&self) -> vl::Decl;
}
