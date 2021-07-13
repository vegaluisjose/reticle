use verilog::ast as vl;

// TODO: this will be eventually be replaced with
// traits in to_verilog.rs file. Therefore, we can
// bypass clippy checks here.
#[allow(clippy::module_name_repetitions)]
pub trait ToDecl {
    fn to_decl(&self) -> vl::Decl;
}
