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
