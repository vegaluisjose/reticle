use crate::{Param, ParamSet, Port, PortSet, Prim, ToPrim};
use derive_more::{Deref, DerefMut, Display, From};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum CascadeOrder {
    #[display(fmt = "FIRST")]
    First,
    #[display(fmt = "MIDDLE")]
    Middle,
    #[display(fmt = "LAST")]
    Last,
    #[display(fmt = "NONE")]
    None,
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum BwMode {
    #[display(fmt = "PARITY_INTERLEAVED")]
    Interleaved,
    #[display(fmt = "PARITY_INDEPENDENT")]
    Independent,
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum RstMode {
    #[display(fmt = "SYNC")]
    Sync,
    #[display(fmt = "ASYNC")]
    Async,
}

#[derive(Clone, Debug, From, Eq)]
pub enum ParamValue {
    CascadeOrder(CascadeOrder),
    BwMode(BwMode),
    RstMode(RstMode),
    Bool(bool),
    #[from(ignore)]
    BoolStr(bool),
    Num(i64),
    Bytes(u32, Vec<u8>),
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Uram(Prim<ParamValue>);

#[derive(Clone, Debug, Default)]
struct UramPrim;

impl PartialEq for ParamValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParamValue::CascadeOrder(_), ParamValue::CascadeOrder(_))
            | (ParamValue::BwMode(_), ParamValue::BwMode(_))
            | (ParamValue::RstMode(_), ParamValue::RstMode(_))
            | (ParamValue::Bool(_), ParamValue::Bool(_))
            | (ParamValue::BoolStr(_), ParamValue::BoolStr(_))
            | (ParamValue::Num(_), ParamValue::Num(_))
            | (ParamValue::Bytes(_, _), ParamValue::Bytes(_, _))
            | (_, _) => false,
        }
    }
}

impl fmt::Display for ParamValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParamValue::Bytes(w, v) => write!(f, "width:{} values:{:?}", w, v),
            _ => write!(f, "{}", self),
        }
    }
}

impl ToPrim<ParamValue> for UramPrim {
    fn to_name(&self) -> String {
        String::from("URAM288")
    }
    fn to_param(&self) -> ParamSet<ParamValue> {
        let mut param = ParamSet::new();
        // TODO: range for this param is 3-15
        // but there is no special types for this
        param.insert(Param {
            name: "AUTO_SLEEP_LATENCY".into(),
            value: 8_i64.into(),
        });
        // TODO: range for this param is 10-10000
        // but there is no special types for this
        param.insert(Param {
            name: "AVG_CONS_INACTIVE_CYCLES".into(),
            value: 10_i64.into(),
        });
        param.insert(Param {
            name: "BWE_MODE_A".into(),
            value: BwMode::Interleaved.into(),
        });
        param.insert(Param {
            name: "BWE_MODE_B".into(),
            value: BwMode::Interleaved.into(),
        });
        param.insert(Param {
            name: "CASCADE_ORDER_A".into(),
            value: CascadeOrder::None.into(),
        });
        param.insert(Param {
            name: "CASCADE_ORDER_B".into(),
            value: CascadeOrder::None.into(),
        });
        param.insert(Param {
            name: "EN_AUTO_SLEEP_MODE".into(),
            value: ParamValue::BoolStr(false),
        });
        param.insert(Param {
            name: "EN_ECC_RD_A".into(),
            value: ParamValue::BoolStr(false),
        });
        param.insert(Param {
            name: "EN_ECC_RD_B".into(),
            value: ParamValue::BoolStr(false),
        });
        param.insert(Param {
            name: "EN_ECC_WR_A".into(),
            value: ParamValue::BoolStr(false),
        });
        param.insert(Param {
            name: "EN_ECC_WR_B".into(),
            value: ParamValue::BoolStr(false),
        });
        param.insert(Param {
            name: "IREG_PRE_A".into(),
            value: ParamValue::BoolStr(false),
        });
        param.insert(Param {
            name: "IREG_PRE_B".into(),
            value: ParamValue::BoolStr(false),
        });
        param.insert(Param {
            name: "IS_CLK_INVERTED".into(),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_EN_A_INVERTED".into(),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_EN_B_INVERTED".into(),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_RDB_WR_A_INVERTED".into(),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_RDB_WR_B_INVERTED".into(),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_RST_A_INVERTED".into(),
            value: false.into(),
        });
        param.insert(Param {
            name: "IS_RST_B_INVERTED".into(),
            value: false.into(),
        });
        param.insert(Param {
            name: "OREG_A".into(),
            value: ParamValue::BoolStr(false),
        });
        param.insert(Param {
            name: "OREG_B".into(),
            value: ParamValue::BoolStr(false),
        });
        param.insert(Param {
            name: "OREG_ECC_A".into(),
            value: ParamValue::BoolStr(false),
        });
        param.insert(Param {
            name: "OREG_ECC_B".into(),
            value: ParamValue::BoolStr(false),
        });
        param.insert(Param {
            name: "REG_CAS_A".into(),
            value: ParamValue::BoolStr(false),
        });
        param.insert(Param {
            name: "REG_CAS_B".into(),
            value: ParamValue::BoolStr(false),
        });
        param.insert(Param {
            name: "RST_MODE_A".into(),
            value: RstMode::Sync.into(),
        });
        param.insert(Param {
            name: "RST_MODE_B".into(),
            value: RstMode::Sync.into(),
        });
        param.insert(Param {
            name: "SELF_ADDR_A".into(),
            value: 0_i64.into(),
        });
        param.insert(Param {
            name: "SELF_ADDR_B".into(),
            value: 0_i64.into(),
        });
        param.insert(Param {
            name: "SELF_MASK_A".into(),
            value: (11, vec![0x7, 0xff]).into(),
        });
        param.insert(Param {
            name: "SELF_MASK_B".into(),
            value: (11, vec![0x7, 0xff]).into(),
        });
        param.insert(Param {
            name: "USE_EXT_CE_A".into(),
            value: ParamValue::BoolStr(false),
        });
        param.insert(Param {
            name: "USE_EXT_CE_B".into(),
            value: ParamValue::BoolStr(false),
        });
        param
    }
    fn to_input(&self) -> PortSet {
        let mut port = PortSet::new();
        port.insert(Port::new("ADDR_A", 23));
        port.insert(Port::new("ADDR_B", 23));
        port.insert(Port::new("BWE_A", 9));
        port.insert(Port::new("BWE_B", 9));
        port.insert(Port::new("CAS_IN_ADDR_A", 23));
        port.insert(Port::new("CAS_IN_ADDR_B", 23));
        port.insert(Port::new("CAS_IN_BWE_A", 9));
        port.insert(Port::new("CAS_IN_BWE_B", 9));
        port.insert(Port::new("CAS_IN_DBITERR_A", 1));
        port.insert(Port::new("CAS_IN_DBITERR_B", 1));
        port.insert(Port::new("CAS_IN_DIN_A", 72));
        port.insert(Port::new("CAS_IN_DIN_B", 72));
        port.insert(Port::new("CAS_IN_DOUT_A", 72));
        port.insert(Port::new("CAS_IN_DOUT_B", 72));
        port.insert(Port::new("CAS_IN_EN_A", 1));
        port.insert(Port::new("CAS_IN_EN_B", 1));
        port.insert(Port::new("CAS_IN_RDACCESS_A", 1));
        port.insert(Port::new("CAS_IN_RDACCESS_B", 1));
        port.insert(Port::new("CAS_IN_RDB_WR_A", 1));
        port.insert(Port::new("CAS_IN_RDB_WR_B", 1));
        port.insert(Port::new("CAS_IN_SBITERR_A", 1));
        port.insert(Port::new("CAS_IN_SBITERR_B", 1));
        port.insert(Port::new("CLK", 1));
        port.insert(Port::new("DIN_A", 72));
        port.insert(Port::new("DIN_B", 72));
        port.insert(Port::new("EN_A", 1));
        port.insert(Port::new("EN_B", 1));
        port.insert(Port::new("INJECT_DBITERR_A", 1));
        port.insert(Port::new("INJECT_DBITERR_B", 1));
        port.insert(Port::new("INJECT_SBITERR_A", 1));
        port.insert(Port::new("INJECT_SBITERR_B", 1));
        port.insert(Port::new("OREG_CE_A", 1));
        port.insert(Port::new("OREG_CE_B", 1));
        port.insert(Port::new("OREG_ECC_CE_A", 1));
        port.insert(Port::new("OREG_ECC_CE_B", 1));
        port.insert(Port::new("RDB_WR_A", 1));
        port.insert(Port::new("RDB_WR_B", 1));
        port.insert(Port::new("RST_A", 1));
        port.insert(Port::new("RST_B", 1));
        port.insert(Port::new("SLEEP", 1));
        port
    }
    fn to_output(&self) -> PortSet {
        let mut port = PortSet::new();
        port.insert(Port::new("CAS_OUT_ADDR_A", 23));
        port.insert(Port::new("CAS_OUT_ADDR_B", 23));
        port.insert(Port::new("CAS_OUT_BWE_A", 9));
        port.insert(Port::new("CAS_OUT_BWE_B", 9));
        port.insert(Port::new("CAS_OUT_DBITERR_A", 1));
        port.insert(Port::new("CAS_OUT_DBITERR_B", 1));
        port.insert(Port::new("CAS_OUT_DIN_A", 72));
        port.insert(Port::new("CAS_OUT_DIN_B", 72));
        port.insert(Port::new("CAS_OUT_DOUT_A", 72));
        port.insert(Port::new("CAS_OUT_DOUT_B", 72));
        port.insert(Port::new("CAS_OUT_EN_A", 1));
        port.insert(Port::new("CAS_OUT_EN_B", 1));
        port.insert(Port::new("CAS_OUT_RDACCESS_A", 1));
        port.insert(Port::new("CAS_OUT_RDACCESS_B", 1));
        port.insert(Port::new("CAS_OUT_RDB_WR_A", 1));
        port.insert(Port::new("CAS_OUT_RDB_WR_B", 1));
        port.insert(Port::new("CAS_OUT_SBITERR_A", 1));
        port.insert(Port::new("CAS_OUT_SBITERR_B", 1));
        port.insert(Port::new("DBITERR_A", 1));
        port.insert(Port::new("DBITERR_B", 1));
        port.insert(Port::new("DOUT_A", 72));
        port.insert(Port::new("DOUT_B", 72));
        port.insert(Port::new("RDACCESS_A", 1));
        port.insert(Port::new("RDACCESS_B", 1));
        port.insert(Port::new("SBITERR_A", 1));
        port.insert(Port::new("SBITERR_B", 1));
        port
    }
}

impl Default for Uram {
    fn default() -> Self {
        let ram = UramPrim;
        Uram(ram.to_prim())
    }
}
