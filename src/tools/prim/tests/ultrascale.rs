use anyhow::Result;
use prim::{Param, ParamSet, PortSet, Prim};
use std::fmt;
use std::iter::FromIterator;

fn test_name<T: Eq + fmt::Debug + fmt::Display>(prim: &Prim<T>, exp: &str) {
    let res = prim.name();
    assert_eq!(res, exp);
}

fn test_param<T: Eq + fmt::Debug + fmt::Display>(prim: &Prim<T>, exp: &ParamSet<T>) {
    let res = prim.param();
    let res = Vec::from_iter(res.iter());
    let exp = Vec::from_iter(exp.iter());
    assert_eq!(res, exp);
}

fn test_input<T: Eq + fmt::Debug + fmt::Display>(prim: &Prim<T>, exp: &[(&str, u32)]) {
    let res = prim.input();
    let exp = PortSet::from(exp);
    assert_eq!(*res, exp);
}

fn test_output<T: Eq + fmt::Debug + fmt::Display>(prim: &Prim<T>, exp: &[(&str, u32)]) {
    let res = prim.output();
    let exp = PortSet::from(exp);
    assert_eq!(*res, exp);
}

mod test_carry {
    use super::*;
    use prim::ultrascale::carry::{Carry, CarryParam, CarryType};

    #[test]
    fn name() {
        let prim = Carry::default();
        test_name(&prim, "CARRY8");
    }

    #[test]
    fn param() {
        let prim = Carry::default();
        let mut param: ParamSet<CarryParam> = ParamSet::new();
        param.insert(Param {
            name: "CARRY_TYPE".to_string(),
            width: None,
            value: CarryType::Single.into(),
        });
        test_param(&prim, &param);
    }

    #[test]
    fn input() {
        let prim = Carry::default();
        let input = [("DI", 8), ("S", 8), ("CI", 1), ("CI_TOP", 1)];
        test_input(&prim, &input);
    }

    #[test]
    fn output() {
        let prim = Carry::default();
        let output = [("O", 8), ("CO", 8)];
        test_output(&prim, &output);
    }

    #[test]
    fn set_param() -> Result<()> {
        let mut prim = Carry::default();
        prim.set_param("CARRY_TYPE", CarryType::Dual)?;
        Ok(())
    }
}

mod test_gnd {
    use super::*;
    use prim::ultrascale::gnd::Gnd;

    #[test]
    fn name() {
        let prim = Gnd::default();
        test_name(&prim, "GND");
    }

    #[test]
    fn param() {
        let prim = Gnd::default();
        let param: ParamSet<_> = ParamSet::new();
        test_param(&prim, &param);
    }

    #[test]
    fn input() {
        let prim = Gnd::default();
        let input = [];
        test_input(&prim, &input);
    }

    #[test]
    fn output() {
        let prim = Gnd::default();
        let output = [("G", 1)];
        test_output(&prim, &output);
    }
}
