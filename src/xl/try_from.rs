use crate::util::errors::Error;
use crate::xl::ast::*;
use std::convert::TryFrom;

impl TryFrom<OptVal> for u64 {
    type Error = Error;
    fn try_from(val: OptVal) -> Result<Self, Self::Error> {
        match val {
            OptVal::UInt(n) => Ok(n),
            _ => Err(Error::new_conv_error("not a uint value")),
        }
    }
}

impl TryFrom<OptVal> for i32 {
    type Error = Error;
    fn try_from(val: OptVal) -> Result<Self, Self::Error> {
        match val {
            OptVal::UInt(a) => {
                if let Ok(b) = i32::try_from(a) {
                    Ok(b)
                } else {
                    Err(Error::new_conv_error("converting u64 to i32"))
                }
            }
            _ => Err(Error::new_conv_error("not a uint value")),
        }
    }
}
