use reticle::frontend::parser::parse_from_file;
use reticle::interp::trace::Trace;
use reticle::interp::ty::Value;
use reticle::interp::Interpreter;

#[cfg(test)]
mod test_scalar_isa {
    use super::*;

    #[test]
    fn test_id() {
        let prog = parse_from_file("examples/isa/scalar/id.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(4));
        trace.enq("a", Value::new_scalar(1));
        trace.enq("y", Value::new_scalar(4));
        trace.enq("y", Value::new_scalar(1));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_const() {
        let prog = parse_from_file("examples/isa/scalar/const.ret");
        let mut trace = Trace::default();
        trace.enq("y", Value::new_scalar(3));
        trace.enq("y", Value::new_scalar(3));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_reg() {
        let prog = parse_from_file("examples/isa/scalar/reg.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(9));
        trace.enq("a", Value::new_scalar(0));
        trace.enq("a", Value::new_scalar(0));
        trace.enq("en", Value::new_scalar(1));
        trace.enq("en", Value::new_scalar(0));
        trace.enq("en", Value::new_scalar(0));
        trace.enq("y", Value::new_scalar(3));
        trace.enq("y", Value::new_scalar(9));
        trace.enq("y", Value::new_scalar(9));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_add() {
        let prog = parse_from_file("examples/isa/scalar/add.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(9));
        trace.enq("b", Value::new_scalar(3));
        trace.enq("y", Value::new_scalar(12));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_sub() {
        let prog = parse_from_file("examples/isa/scalar/sub.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(9));
        trace.enq("b", Value::new_scalar(3));
        trace.enq("y", Value::new_scalar(6));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_mul() {
        let prog = parse_from_file("examples/isa/scalar/mul.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(9));
        trace.enq("b", Value::new_scalar(3));
        trace.enq("y", Value::new_scalar(27));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_not() {
        let prog = parse_from_file("examples/isa/scalar/not.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(3)); // 0b00000011
        trace.enq("y", Value::new_scalar(-4)); // 0b11111100
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_and() {
        let prog = parse_from_file("examples/isa/scalar/and.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(3));
        trace.enq("b", Value::new_scalar(3));
        trace.enq("y", Value::new_scalar(3));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_nand() {
        let prog = parse_from_file("examples/isa/scalar/nand.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(15));
        trace.enq("b", Value::new_scalar(15));
        trace.enq("y", Value::new_scalar(-16));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_or() {
        let prog = parse_from_file("examples/isa/scalar/or.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(3));
        trace.enq("b", Value::new_scalar(8));
        trace.enq("y", Value::new_scalar(11));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_nor() {
        let prog = parse_from_file("examples/isa/scalar/nor.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(7));
        trace.enq("b", Value::new_scalar(8));
        trace.enq("y", Value::new_scalar(-16));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_xor() {
        let prog = parse_from_file("examples/isa/scalar/xor.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(3));
        trace.enq("b", Value::new_scalar(12));
        trace.enq("y", Value::new_scalar(15));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_xnor() {
        let prog = parse_from_file("examples/isa/scalar/xnor.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(3));
        trace.enq("b", Value::new_scalar(12));
        trace.enq("y", Value::new_scalar(-16));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_mux() {
        let prog = parse_from_file("examples/isa/scalar/mux.ret");
        let mut trace = Trace::default();
        trace.enq("cond", Value::new_scalar(0));
        trace.enq("cond", Value::new_scalar(1));
        trace.enq("t", Value::new_scalar(2));
        trace.enq("t", Value::new_scalar(2));
        trace.enq("f", Value::new_scalar(6));
        trace.enq("f", Value::new_scalar(6));
        trace.enq("y", Value::new_scalar(6));
        trace.enq("y", Value::new_scalar(2));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_eq() {
        let prog = parse_from_file("examples/isa/scalar/eq.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(3));
        trace.enq("b", Value::new_scalar(3));
        trace.enq("y", Value::new_scalar(1));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_neq() {
        let prog = parse_from_file("examples/isa/scalar/neq.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(3));
        trace.enq("b", Value::new_scalar(3));
        trace.enq("y", Value::new_scalar(0));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_gt() {
        let prog = parse_from_file("examples/isa/scalar/gt.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(1));
        trace.enq("b", Value::new_scalar(3));
        trace.enq("y", Value::new_scalar(0));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_lt() {
        let prog = parse_from_file("examples/isa/scalar/lt.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(1));
        trace.enq("b", Value::new_scalar(3));
        trace.enq("y", Value::new_scalar(1));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_ge() {
        let prog = parse_from_file("examples/isa/scalar/ge.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(4));
        trace.enq("b", Value::new_scalar(4));
        trace.enq("y", Value::new_scalar(1));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_le() {
        let prog = parse_from_file("examples/isa/scalar/le.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(4));
        trace.enq("b", Value::new_scalar(4));
        trace.enq("y", Value::new_scalar(1));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }
}

#[cfg(test)]
mod test_vector_isa {
    use super::*;

    #[test]
    fn test_vid_v4() {
        let prog = parse_from_file("examples/isa/vector/vid_v4.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::from(vec![4, 3, 1, 0]));
        trace.enq("a", Value::from(vec![11, 2, 77, 90]));
        trace.enq("y", Value::from(vec![4, 3, 1, 0]));
        trace.enq("y", Value::from(vec![11, 2, 77, 90]));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_vreg_v4() {
        let prog = parse_from_file("examples/isa/vector/vreg_v4.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::from(vec![4, 2, 2, 1]));
        trace.enq("a", Value::from(vec![0, 0, 0, 0]));
        trace.enq("a", Value::from(vec![0, 0, 0, 0]));
        trace.enq("en", Value::new_scalar(1));
        trace.enq("en", Value::new_scalar(0));
        trace.enq("en", Value::new_scalar(0));
        trace.enq("y", Value::from(vec![3, 0, 1, 5]));
        trace.enq("y", Value::from(vec![4, 2, 2, 1]));
        trace.enq("y", Value::from(vec![4, 2, 2, 1]));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_vadd_v4() {
        let prog = parse_from_file("examples/isa/vector/vadd_v4.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::from(vec![-4, 2, 2, 1]));
        trace.enq("b", Value::from(vec![1, 3, 0, 1]));
        trace.enq("y", Value::from(vec![-3, 5, 2, 2]));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_vsub_v4() {
        let prog = parse_from_file("examples/isa/vector/vsub_v4.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::from(vec![-4, 2, 2, 1]));
        trace.enq("b", Value::from(vec![1, 3, 0, 1]));
        trace.enq("y", Value::from(vec![-5, -1, 2, 0]));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_vmul_v4() {
        let prog = parse_from_file("examples/isa/vector/vmul_v4.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::from(vec![-4, 2, 2, 1]));
        trace.enq("b", Value::from(vec![1, 3, 0, 1]));
        trace.enq("y", Value::from(vec![-4, 6, 0, 1]));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_vnot_v4() {
        let prog = parse_from_file("examples/isa/vector/vnot_v4.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::from(vec![-4, 3, 1, 0]));
        trace.enq("y", Value::from(vec![3, -4, -2, -1]));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_vand_v4() {
        let prog = parse_from_file("examples/isa/vector/vand_v4.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::from(vec![4, 15, 1, 0]));
        trace.enq("b", Value::from(vec![4, 3, 0, 0]));
        trace.enq("y", Value::from(vec![4, 3, 0, 0]));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_vnand_v4() {
        let prog = parse_from_file("examples/isa/vector/vnand_v4.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::from(vec![4, 15, 1, 0]));
        trace.enq("b", Value::from(vec![4, 15, 0, 0]));
        trace.enq("y", Value::from(vec![-5, -16, -1, -1]));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_vor_v4() {
        let prog = parse_from_file("examples/isa/vector/vor_v4.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::from(vec![4, 8, 1, 0]));
        trace.enq("b", Value::from(vec![4, 3, 0, 0]));
        trace.enq("y", Value::from(vec![4, 11, 1, 0]));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_vnor_v4() {
        let prog = parse_from_file("examples/isa/vector/vnor_v4.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::from(vec![7, -2, 1, 0]));
        trace.enq("b", Value::from(vec![8, 1, 0, 0]));
        trace.enq("y", Value::from(vec![-16, 0, -2, -1]));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_xor_v4() {
        let prog = parse_from_file("examples/isa/vector/vxor_v4.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::from(vec![3, 0, 1, 0]));
        trace.enq("b", Value::from(vec![12, 3, 1, 0]));
        trace.enq("y", Value::from(vec![15, 3, 0, 0]));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_xnor_v4() {
        let prog = parse_from_file("examples/isa/vector/vxnor_v4.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::from(vec![7, 1, 1, 0]));
        trace.enq("b", Value::from(vec![8, 1, 0, 0]));
        trace.enq("y", Value::from(vec![-16, -1, -2, -1]));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }
}
