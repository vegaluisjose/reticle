use crate::ast::*;

impl Default for ExprCoord {
    fn default() -> Self {
        ExprCoord::Any
    }
}

impl Default for Loc {
    fn default() -> Self {
        Loc {
            prim: Prim::Any,
            x: ExprCoord::default(),
            y: ExprCoord::default(),
        }
    }
}
