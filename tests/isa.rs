use reticle::frontend::parser::parse_from_file;
use reticle::interp::trace::Trace;
use reticle::interp::ty::Value;
use reticle::interp::Interpreter;

#[cfg(test)]
mod test_scalar_instr {
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

    //     #[test]
    //     fn test_sub() {
    //         let prog = parse_from_file("examples/isa/sub.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("a", 9);
    //         trace.enq("b", 7);
    //         trace.enq("y", 2);
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }

    //     #[test]
    //     fn test_mul() {
    //         let prog = parse_from_file("examples/isa/mul.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("a", 3);
    //         trace.enq("b", 5);
    //         trace.enq("y", 15);
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }

    //     #[test]
    //     fn test_not() {
    //         let prog = parse_from_file("examples/isa/not.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("a", 3); // 0b00000011
    //         trace.enq("y", -4); // 0b11111100
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }

    //     #[test]
    //     fn test_and() {
    //         let prog = parse_from_file("examples/isa/and.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("a", 3);
    //         trace.enq("b", 3);
    //         trace.enq("y", 3);
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }

    //     #[test]
    //     fn test_nand() {
    //         let prog = parse_from_file("examples/isa/nand.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("a", 15);
    //         trace.enq("b", 15);
    //         trace.enq("y", -16);
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }

    //     #[test]
    //     fn test_or() {
    //         let prog = parse_from_file("examples/isa/or.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("a", 3);
    //         trace.enq("b", 8);
    //         trace.enq("y", 11);
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }

    //     #[test]
    //     fn test_nor() {
    //         let prog = parse_from_file("examples/isa/nor.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("a", 7);
    //         trace.enq("b", 8);
    //         trace.enq("y", -16);
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }

    //     #[test]
    //     fn test_xor() {
    //         let prog = parse_from_file("examples/isa/xor.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("a", 3);
    //         trace.enq("b", 12);
    //         trace.enq("y", 15);
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }

    //     #[test]
    //     fn test_xnor() {
    //         let prog = parse_from_file("examples/isa/xnor.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("a", 3);
    //         trace.enq("b", 12);
    //         trace.enq("y", -16);
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }

    //     #[test]
    //     fn test_mux() {
    //         let prog = parse_from_file("examples/isa/mux.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("cond", 0);
    //         trace.enq("t", 2);
    //         trace.enq("f", 6);
    //         trace.enq("y", 6);
    //         trace.enq("cond", 1);
    //         trace.enq("t", 2);
    //         trace.enq("f", 6);
    //         trace.enq("y", 2);
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }

    //     #[test]
    //     fn test_eq() {
    //         let prog = parse_from_file("examples/isa/eq.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("a", 3);
    //         trace.enq("b", 3);
    //         trace.enq("y", 1);
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }

    //     #[test]
    //     fn test_neq() {
    //         let prog = parse_from_file("examples/isa/neq.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("a", 3);
    //         trace.enq("b", 3);
    //         trace.enq("y", 0);
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }

    //     #[test]
    //     fn test_gt() {
    //         let prog = parse_from_file("examples/isa/gt.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("a", 1);
    //         trace.enq("b", 3);
    //         trace.enq("y", 0);
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }

    //     #[test]
    //     fn test_lt() {
    //         let prog = parse_from_file("examples/isa/lt.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("a", 1);
    //         trace.enq("b", 3);
    //         trace.enq("y", 1);
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }

    //     #[test]
    //     fn test_ge() {
    //         let prog = parse_from_file("examples/isa/ge.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("a", 4);
    //         trace.enq("b", 4);
    //         trace.enq("y", 1);
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }

    //     #[test]
    //     fn test_le() {
    //         let prog = parse_from_file("examples/isa/le.ret");
    //         let mut trace = Trace::default();
    //         trace.enq("a", 4);
    //         trace.enq("b", 4);
    //         trace.enq("y", 1);
    //         assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    //     }
}

#[cfg(test)]
mod test_vector_instr {
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
}
