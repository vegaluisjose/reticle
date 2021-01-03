use crate::ml::ast as ml;
use crate::verilog::ast as vl;

impl From<ml::OpMach> for vl::Id {
    fn from(op: ml::OpMach) -> Self {
        match op {
            ml::OpMach::Lut1 => "LUT1".to_string(),
            ml::OpMach::Lut2 => "LUT2".to_string(),
            ml::OpMach::Lut3 => "LUT3".to_string(),
            ml::OpMach::Lut4 => "LUT4".to_string(),
            ml::OpMach::Lut5 => "LUT5".to_string(),
            ml::OpMach::Lut6 => "LUT6".to_string(),
            ml::OpMach::Fdre => "FDRE".to_string(),
            ml::OpMach::Fdse => "FDSE".to_string(),
            ml::OpMach::Dsp => "DSP48E2".to_string(),
            ml::OpMach::Carry => "CARRY8".to_string(),
        }
    }
}

impl From<ml::BelLut> for vl::Id {
    fn from(bel: ml::BelLut) -> Self {
        match bel {
            ml::BelLut::A5 => "A5LUT".to_string(),
            ml::BelLut::B5 => "B5LUT".to_string(),
            ml::BelLut::C5 => "C5LUT".to_string(),
            ml::BelLut::D5 => "D5LUT".to_string(),
            ml::BelLut::E5 => "E5LUT".to_string(),
            ml::BelLut::F5 => "F5LUT".to_string(),
            ml::BelLut::G5 => "G5LUT".to_string(),
            ml::BelLut::H5 => "H5LUT".to_string(),
            ml::BelLut::A6 => "A6LUT".to_string(),
            ml::BelLut::B6 => "B6LUT".to_string(),
            ml::BelLut::C6 => "C6LUT".to_string(),
            ml::BelLut::D6 => "D6LUT".to_string(),
            ml::BelLut::E6 => "E6LUT".to_string(),
            ml::BelLut::F6 => "F6LUT".to_string(),
            ml::BelLut::G6 => "G6LUT".to_string(),
            ml::BelLut::H6 => "H6LUT".to_string(),
        }
    }
}

impl From<ml::BelReg> for vl::Id {
    fn from(bel: ml::BelReg) -> Self {
        match bel {
            ml::BelReg::A => "AFF".to_string(),
            ml::BelReg::B => "BFF".to_string(),
            ml::BelReg::C => "CFF".to_string(),
            ml::BelReg::D => "DFF".to_string(),
            ml::BelReg::E => "EFF".to_string(),
            ml::BelReg::F => "FFF".to_string(),
            ml::BelReg::G => "GFF".to_string(),
            ml::BelReg::H => "HFF".to_string(),
            ml::BelReg::A2 => "AFF2".to_string(),
            ml::BelReg::B2 => "BFF2".to_string(),
            ml::BelReg::C2 => "CFF2".to_string(),
            ml::BelReg::D2 => "DFF2".to_string(),
            ml::BelReg::E2 => "EFF2".to_string(),
            ml::BelReg::F2 => "FFF2".to_string(),
            ml::BelReg::G2 => "GFF2".to_string(),
            ml::BelReg::H2 => "HFF2".to_string(),
        }
    }
}

impl From<ml::BelCarry> for vl::Id {
    fn from(bel: ml::BelCarry) -> Self {
        match bel {
            ml::BelCarry::Carry4 => "CARRY4".to_string(),
            ml::BelCarry::Carry8 => "CARRY8".to_string(),
        }
    }
}

impl From<ml::BelDsp> for vl::Id {
    fn from(bel: ml::BelDsp) -> Self {
        match bel {
            ml::BelDsp::Alu => "DSP_ALU".to_string(),
        }
    }
}

impl From<ml::Bel> for vl::Id {
    fn from(bel: ml::Bel) -> Self {
        match bel {
            ml::Bel::Lut(lut) => lut.into(),
            ml::Bel::Reg(reg) => reg.into(),
            ml::Bel::Carry(carry) => carry.into(),
            ml::Bel::Dsp(dsp) => dsp.into(),
        }
    }
}

impl From<ml::Bel> for vl::AttributeTy {
    fn from(bel: ml::Bel) -> Self {
        let val = vl::Id::from(bel);
        vl::AttributeTy::new_stmt("BEL", &val)
    }
}

impl From<ml::Loc> for vl::AttributeTy {
    fn from(loc: ml::Loc) -> Self {
        let val = match loc.bel() {
            ml::Bel::Dsp(_) => format!("DSP48E2_X{}Y{}", loc.x(), loc.y()),
            _ => format!("SLICE_X{}Y{}", loc.x(), loc.y()),
        };
        vl::AttributeTy::new_stmt("LOC", &val)
    }
}

impl From<ml::Loc> for vl::Attribute {
    fn from(loc: ml::Loc) -> Self {
        let mut attr = vl::Attribute::default();
        let site = vl::AttributeTy::from(loc.bel().clone());
        let bel = vl::AttributeTy::from(loc);
        attr.add_attr(site);
        attr.add_attr(bel);
        attr
    }
}
