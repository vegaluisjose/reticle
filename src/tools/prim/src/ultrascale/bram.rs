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
pub enum ClockDomains {
    #[display(fmt = "COMMON")]
    Common,
    #[display(fmt = "INDEPENDENT")]
    Independent,
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum CollisionCheck {
    #[display(fmt = "ALL")]
    All,
    #[display(fmt = "GENERATE_X_ONLY")]
    GenX,
    #[display(fmt = "NONE")]
    None,
    #[display(fmt = "WARNING_ONLY")]
    Warning,
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum FilePath {
    #[display(fmt = "{}", _0)]
    Some(String),
    #[display(fmt = "NONE")]
    None,
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum RstRegPriority {
    #[display(fmt = "RSTREG")]
    RstReg,
    #[display(fmt = "REGCE")]
    RegCe,
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
pub enum WriteMode {
    #[display(fmt = "WRITE_FIRST")]
    WriteFirst,
    #[display(fmt = "NO_CHANGE")]
    NoChange,
    #[display(fmt = "READ_FIRST")]
    ReadFirst,
}

#[derive(Clone, Debug, From, Eq)]
pub enum ParamValue {
    CascadeOrder(CascadeOrder),
    ClockDomains(ClockDomains),
    CollisionCheck(CollisionCheck),
    Bool(bool),
    #[from(ignore)]
    BoolNum(bool),
    #[from(ignore)]
    BoolStr(bool),
    Bytes(u32, Vec<u8>),
    Num(u32),
    FilePath(FilePath),
    RstRegPriority(RstRegPriority),
    WriteMode(WriteMode),
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Bram(Prim<ParamValue>);

#[derive(Clone, Debug, Default)]
struct BramPrim;

impl PartialEq for ParamValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParamValue::CascadeOrder(_), ParamValue::CascadeOrder(_))
            | (ParamValue::ClockDomains(_), ParamValue::ClockDomains(_))
            | (ParamValue::CollisionCheck(_), ParamValue::CollisionCheck(_))
            | (ParamValue::Bool(_), ParamValue::Bool(_))
            | (ParamValue::BoolStr(_), ParamValue::BoolStr(_))
            | (ParamValue::Bytes(_, _), ParamValue::Bytes(_, _))
            | (ParamValue::Num(_), ParamValue::Num(_))
            | (ParamValue::FilePath(_), ParamValue::FilePath(_))
            | (ParamValue::RstRegPriority(_), ParamValue::RstRegPriority(_))
            | (ParamValue::WriteMode(_), ParamValue::WriteMode(_)) => true,
            (_, _) => false,
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

const PARAM: [(&str, ParamValue); 32] = [
    (
        "CASCADE_ORDER_A",
        ParamValue::CascadeOrder(CascadeOrder::None),
    ),
    (
        "CASCADE_ORDER_B",
        ParamValue::CascadeOrder(CascadeOrder::None),
    ),
    (
        "CLOCK_DOMAINS",
        ParamValue::ClockDomains(ClockDomains::Independent),
    ),
    (
        "SIM_COLLISION_CHECK",
        ParamValue::CollisionCheck(CollisionCheck::All),
    ),
    ("DOA_REG", ParamValue::BoolNum(false)),
    ("DOB_REG", ParamValue::BoolNum(false)),
    ("ENADDRENA", ParamValue::BoolStr(false)),
    ("ENADDRENB", ParamValue::BoolStr(false)),
    ("INIT_A", ParamValue::Bytes(18, vec![])),
    ("INIT_B", ParamValue::Bytes(18, vec![])),
    ("INIT_FILE", ParamValue::FilePath(FilePath::None)),
    ("IS_CLKARDCLK_INVERTED", ParamValue::Bool(false)),
    ("IS_CLKBWRCLK_INVERTED", ParamValue::Bool(false)),
    ("IS_ENARDEN_INVERTED", ParamValue::Bool(false)),
    ("IS_ENBWREN_INVERTED", ParamValue::Bool(false)),
    ("IS_RSTRAMARSTRAM_INVERTED", ParamValue::Bool(false)),
    ("IS_RSTRAMB_INVERTED", ParamValue::Bool(false)),
    ("IS_RSTREGARSTREG_INVERTED", ParamValue::Bool(false)),
    ("IS_RSTREGB_INVERTED", ParamValue::Bool(false)),
    ("RDADDRCHANGEA", ParamValue::BoolStr(false)),
    ("RDADDRCHANGEB", ParamValue::BoolStr(false)),
    ("READ_WIDTH_A", ParamValue::Num(0)),
    ("READ_WIDTH_B", ParamValue::Num(0)),
    ("WRITE_WIDTH_A", ParamValue::Num(0)),
    ("WRITE_WIDTH_B", ParamValue::Num(0)),
    (
        "RSTREG_PRIORITY_A",
        ParamValue::RstRegPriority(RstRegPriority::RstReg),
    ),
    (
        "RSTREG_PRIORITY_B",
        ParamValue::RstRegPriority(RstRegPriority::RstReg),
    ),
    ("SRVAL_A", ParamValue::Num(0)),
    ("SRVAL_B", ParamValue::Num(0)),
    ("SLEEP_ASYNC", ParamValue::BoolStr(false)),
    ("WRITE_MODE_A", ParamValue::WriteMode(WriteMode::NoChange)),
    ("WRITE_MODE_B", ParamValue::WriteMode(WriteMode::NoChange)),
];

impl ToPrim<ParamValue> for BramPrim {
    fn to_name(&self) -> String {
        String::from("RAMB18E2")
    }
    fn to_param(&self) -> ParamSet<ParamValue> {
        let mut param = ParamSet::new();
        for p in &PARAM {
            param.insert(Param {
                name: p.0.into(),
                value: p.1.clone(),
            });
        }
        for i in 0..8 {
            let name = format!("INITP_{:02X}", i);
            param.insert(Param {
                name,
                value: (256, vec![0; 32]).into(),
            });
        }
        for i in 0..64 {
            let name = format!("INIT_{:02X}", i);
            param.insert(Param {
                name,
                value: (256, vec![0; 32]).into(),
            });
        }
        param
    }
    fn to_input(&self) -> PortSet {
        let mut port = PortSet::new();
        port.insert(Port::new("CASDIMUXA", 1));
        port.insert(Port::new("CASDIMUXB", 1));
        port.insert(Port::new("CASDINA", 16));
        port.insert(Port::new("CASDINB", 16));
        port.insert(Port::new("CASDINPA", 2));
        port.insert(Port::new("CASDINPB", 2));
        port.insert(Port::new("CASDOMUXA", 1));
        port.insert(Port::new("CASDOMUXB", 1));
        port.insert(Port::new("CASDOMUXEN_A", 1));
        port.insert(Port::new("CASDOMUXEN_B", 1));
        port.insert(Port::new("CASOREGIMUXA", 1));
        port.insert(Port::new("CASOREGIMUXB", 1));
        port.insert(Port::new("CASOREGIMUXEN_A", 1));
        port.insert(Port::new("CASOREGIMUXEN_B", 1));
        port.insert(Port::new("ADDRARDADDR", 14));
        port.insert(Port::new("ADDRENA", 1));
        port.insert(Port::new("CLKARDCLK", 1));
        port.insert(Port::new("ENARDEN", 1));
        port.insert(Port::new("REGCEAREGCE", 1));
        port.insert(Port::new("RSTRAMARSTRAM", 1));
        port.insert(Port::new("RSTREGARSTREG", 1));
        port.insert(Port::new("WEA", 2));
        port.insert(Port::new("DINADIN", 16));
        port.insert(Port::new("DINPADINP", 2));
        port.insert(Port::new("ADDRBWRADDR", 14));
        port.insert(Port::new("ADDRENB", 1));
        port.insert(Port::new("CLKBWRCLK", 1));
        port.insert(Port::new("ENBWREN", 1));
        port.insert(Port::new("REGCEB", 1));
        port.insert(Port::new("RSTRAMB", 1));
        port.insert(Port::new("RSTREGB", 1));
        port.insert(Port::new("SLEEP", 1));
        port.insert(Port::new("WEBWE", 4));
        port.insert(Port::new("DINBDIN", 16));
        port.insert(Port::new("DINPBDINP", 2));
        port
    }
    fn to_output(&self) -> PortSet {
        let mut port = PortSet::new();
        port.insert(Port::new("CASDOUTA", 16));
        port.insert(Port::new("CASDOUTB", 16));
        port.insert(Port::new("CASDOUTPA", 2));
        port.insert(Port::new("CASDOUTPB", 2));
        port.insert(Port::new("DOUTADOUT", 16));
        port.insert(Port::new("DOUTPADOUTP", 2));
        port.insert(Port::new("DOUTBDOUT", 16));
        port.insert(Port::new("DOUTPBDOUTP", 2));
        port
    }
}

impl Default for Bram {
    fn default() -> Self {
        let ram = BramPrim;
        Bram(ram.to_prim())
    }
}
