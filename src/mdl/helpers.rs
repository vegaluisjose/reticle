use crate::mdl::ast::*;

impl LocLut {
    pub fn bel(&self) -> &BelLut {
        &self.bel
    }
    pub fn x(&self) -> &ExprCoord {
        &self.x
    }
    pub fn y(&self) -> &ExprCoord {
        &self.y
    }
}
