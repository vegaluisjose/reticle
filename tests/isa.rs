use reticle::frontend::parser::parse_from_file;
use reticle::interp::trace::Trace;
use reticle::interp::Interpreter;

#[cfg(test)]
mod test_scalar_isa {
    use super::*;

    #[test]
    fn test_id_i8_i8() {
        let prog = parse_from_file("examples/isa/scalar/id_i8_i8.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 4);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("y", 4);
        trace.enq_scalar("y", 1);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_const_i8() {
        let prog = parse_from_file("examples/isa/scalar/const_i8.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("y", 3);
        trace.enq_scalar("y", 3);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_reg_i8_i8_b() {
        let prog = parse_from_file("examples/isa/scalar/reg_i8_i8_b.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 9);
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 0);
        trace.enq_scalar("en", 1);
        trace.enq_scalar("en", 0);
        trace.enq_scalar("en", 0);
        trace.enq_scalar("y", 3);
        trace.enq_scalar("y", 9);
        trace.enq_scalar("y", 9);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_add() {
        let prog = parse_from_file("examples/isa/scalar/add.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 9);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("y", 12);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_sub() {
        let prog = parse_from_file("examples/isa/scalar/sub.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 9);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("y", 6);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_mul_i8_i8_i8() {
        let prog = parse_from_file("examples/isa/scalar/mul_i8_i8_i8.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 9);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("y", 27);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_not_i8_i8() {
        let prog = parse_from_file("examples/isa/scalar/not_i8_i8.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 3); // 0b00000011
        trace.enq_scalar("y", -4); // 0b11111100
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_not_b_b() {
        let prog = parse_from_file("examples/isa/scalar/not_b_b.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("y", 1);
        trace.enq_scalar("y", 0);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_and_i8_i8_i8() {
        let prog = parse_from_file("examples/isa/scalar/and_i8_i8_i8.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 3);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("y", 3);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_and_b_b_b() {
        let prog = parse_from_file("examples/isa/scalar/and_b_b_b.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("b", 1);
        trace.enq_scalar("b", 1);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 1);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_nand_i8_i8_i8() {
        let prog = parse_from_file("examples/isa/scalar/nand_i8_i8_i8.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 15);
        trace.enq_scalar("b", 15);
        trace.enq_scalar("y", -16);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_nand_b_b_b() {
        let prog = parse_from_file("examples/isa/scalar/nand_b_b_b.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("b", 1);
        trace.enq_scalar("b", 1);
        trace.enq_scalar("y", 1);
        trace.enq_scalar("y", 1);
        trace.enq_scalar("y", 1);
        trace.enq_scalar("y", 0);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_or_i8_i8_i8() {
        let prog = parse_from_file("examples/isa/scalar/or_i8_i8_i8.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 3);
        trace.enq_scalar("b", 8);
        trace.enq_scalar("y", 11);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_or_b_b_b() {
        let prog = parse_from_file("examples/isa/scalar/or_b_b_b.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("b", 1);
        trace.enq_scalar("b", 1);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 1);
        trace.enq_scalar("y", 1);
        trace.enq_scalar("y", 1);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_nor_i8_i8_i8() {
        let prog = parse_from_file("examples/isa/scalar/nor_i8_i8_i8.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 7);
        trace.enq_scalar("b", 8);
        trace.enq_scalar("y", -16);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_nor_b_b_b() {
        let prog = parse_from_file("examples/isa/scalar/nor_b_b_b.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("b", 1);
        trace.enq_scalar("b", 1);
        trace.enq_scalar("y", 1);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 0);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_xor_i8_i8_i8() {
        let prog = parse_from_file("examples/isa/scalar/xor_i8_i8_i8.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 3);
        trace.enq_scalar("b", 12);
        trace.enq_scalar("y", 15);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_xor_b_b_b() {
        let prog = parse_from_file("examples/isa/scalar/xor_b_b_b.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("b", 1);
        trace.enq_scalar("b", 1);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 1);
        trace.enq_scalar("y", 1);
        trace.enq_scalar("y", 0);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_xnor_i8_i8_i8() {
        let prog = parse_from_file("examples/isa/scalar/xnor_i8_i8_i8.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 3);
        trace.enq_scalar("b", 12);
        trace.enq_scalar("y", -16);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_xnor_b_b_b() {
        let prog = parse_from_file("examples/isa/scalar/xnor_b_b_b.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("b", 1);
        trace.enq_scalar("b", 1);
        trace.enq_scalar("y", 1);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 1);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_mux_i8_b_i8_i8() {
        let prog = parse_from_file("examples/isa/scalar/mux_i8_b_i8_i8.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("cond", 0);
        trace.enq_scalar("cond", 1);
        trace.enq_scalar("t", 2);
        trace.enq_scalar("t", 2);
        trace.enq_scalar("f", 6);
        trace.enq_scalar("f", 6);
        trace.enq_scalar("y", 6);
        trace.enq_scalar("y", 2);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_mux_b_b_b_b() {
        let prog = parse_from_file("examples/isa/scalar/mux_b_b_b_b.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("cond", 0);
        trace.enq_scalar("cond", 1);
        trace.enq_scalar("t", 1);
        trace.enq_scalar("t", 1);
        trace.enq_scalar("f", 0);
        trace.enq_scalar("f", 0);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 1);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_eq_b_i8_i8() {
        let prog = parse_from_file("examples/isa/scalar/eq_b_i8_i8.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 5);
        trace.enq_scalar("b", 5);
        trace.enq_scalar("y", 1);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_eq_b_b_b() {
        let prog = parse_from_file("examples/isa/scalar/eq_b_b_b.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 0);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("y", 1);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_neq_b_i8_i8() {
        let prog = parse_from_file("examples/isa/scalar/neq_b_i8_i8.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 3);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("y", 0);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_neq_b_b_b() {
        let prog = parse_from_file("examples/isa/scalar/neq_b_b_b.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 0);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("y", 0);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_gt() {
        let prog = parse_from_file("examples/isa/scalar/gt.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 1);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("y", 0);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_lt() {
        let prog = parse_from_file("examples/isa/scalar/lt.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 1);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("y", 1);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_ge() {
        let prog = parse_from_file("examples/isa/scalar/ge.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 4);
        trace.enq_scalar("b", 4);
        trace.enq_scalar("y", 1);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_le() {
        let prog = parse_from_file("examples/isa/scalar/le.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 4);
        trace.enq_scalar("b", 4);
        trace.enq_scalar("y", 1);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_shl() {
        let prog = parse_from_file("examples/isa/scalar/shl.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 4);
        trace.enq_scalar("y", 8);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_shr() {
        let prog = parse_from_file("examples/isa/scalar/shr.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 3);
        trace.enq_scalar("y", 1);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }
}

#[cfg(test)]
mod test_vector_isa {
    use super::*;

    #[test]
    fn test_vid_v4() {
        let prog = parse_from_file("examples/isa/vector/vid_v4.ret");
        let mut trace = Trace::default();
        trace.enq_vector("a", vec![4, 3, 1, 0]);
        trace.enq_vector("a", vec![11, 2, 77, 90]);
        trace.enq_vector("y", vec![4, 3, 1, 0]);
        trace.enq_vector("y", vec![11, 2, 77, 90]);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_vreg_v4() {
        let prog = parse_from_file("examples/isa/vector/vreg_v4.ret");
        let mut trace = Trace::default();
        trace.enq_vector("a", vec![4, 2, 2, 1]);
        trace.enq_vector("a", vec![0, 0, 0, 0]);
        trace.enq_vector("a", vec![0, 0, 0, 0]);
        trace.enq_scalar("en", 1);
        trace.enq_scalar("en", 0);
        trace.enq_scalar("en", 0);
        trace.enq_vector("y", vec![3, 0, 1, 5]);
        trace.enq_vector("y", vec![4, 2, 2, 1]);
        trace.enq_vector("y", vec![4, 2, 2, 1]);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_add_i8v4_i8v4_i8v4() {
        let prog = parse_from_file("examples/isa/vector/add_i8v4_i8v4_i8v4.ret");
        let mut trace = Trace::default();
        trace.enq_vector("a", vec![-4, 2, 2, 1]);
        trace.enq_vector("b", vec![1, 3, 0, 1]);
        trace.enq_vector("y", vec![-3, 5, 2, 2]);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_vsub_v4() {
        let prog = parse_from_file("examples/isa/vector/vsub_v4.ret");
        let mut trace = Trace::default();
        trace.enq_vector("a", vec![-4, 2, 2, 1]);
        trace.enq_vector("b", vec![1, 3, 0, 1]);
        trace.enq_vector("y", vec![-5, -1, 2, 0]);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_vnot_v4() {
        let prog = parse_from_file("examples/isa/vector/vnot_v4.ret");
        let mut trace = Trace::default();
        trace.enq_vector("a", vec![-4, 3, 1, 0]);
        trace.enq_vector("y", vec![3, -4, -2, -1]);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_vand_v4() {
        let prog = parse_from_file("examples/isa/vector/vand_v4.ret");
        let mut trace = Trace::default();
        trace.enq_vector("a", vec![4, 15, 1, 0]);
        trace.enq_vector("b", vec![4, 3, 0, 0]);
        trace.enq_vector("y", vec![4, 3, 0, 0]);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_vnand_v4() {
        let prog = parse_from_file("examples/isa/vector/vnand_v4.ret");
        let mut trace = Trace::default();
        trace.enq_vector("a", vec![4, 15, 1, 0]);
        trace.enq_vector("b", vec![4, 15, 0, 0]);
        trace.enq_vector("y", vec![-5, -16, -1, -1]);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_vor_v4() {
        let prog = parse_from_file("examples/isa/vector/vor_v4.ret");
        let mut trace = Trace::default();
        trace.enq_vector("a", vec![4, 8, 1, 0]);
        trace.enq_vector("b", vec![4, 3, 0, 0]);
        trace.enq_vector("y", vec![4, 11, 1, 0]);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_vnor_v4() {
        let prog = parse_from_file("examples/isa/vector/vnor_v4.ret");
        let mut trace = Trace::default();
        trace.enq_vector("a", vec![7, -2, 1, 0]);
        trace.enq_vector("b", vec![8, 1, 0, 0]);
        trace.enq_vector("y", vec![-16, 0, -2, -1]);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_xor_v4() {
        let prog = parse_from_file("examples/isa/vector/vxor_v4.ret");
        let mut trace = Trace::default();
        trace.enq_vector("a", vec![3, 0, 1, 0]);
        trace.enq_vector("b", vec![12, 3, 1, 0]);
        trace.enq_vector("y", vec![15, 3, 0, 0]);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_xnor_v4() {
        let prog = parse_from_file("examples/isa/vector/vxnor_v4.ret");
        let mut trace = Trace::default();
        trace.enq_vector("a", vec![7, 1, 1, 0]);
        trace.enq_vector("b", vec![8, 1, 0, 0]);
        trace.enq_vector("y", vec![-16, -1, -2, -1]);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }
}
