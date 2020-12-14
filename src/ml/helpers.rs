use crate::ml::ast::*;

impl Loc {
    pub fn bel(&self) -> Option<&Bel> {
        self.bel.as_ref()
    }
    pub fn x(&self) -> &ExprCoord {
        &self.x
    }
    pub fn y(&self) -> &ExprCoord {
        &self.y
    }
}
