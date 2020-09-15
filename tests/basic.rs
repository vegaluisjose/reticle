use reticle::frontend::parser::parse_from_file;
use reticle::interp::trace::Trace;
use reticle::interp::Interpreter;

#[cfg(test)]
mod test_basic {
    use super::*;

    #[test]
    fn test_vadd_const() {
        let prog = parse_from_file("examples/basic/vadd_const.ret");
        let mut trace = Trace::default();
        trace.enq_vector("a", vec![-4, 2, 0, 1]);
        trace.enq_vector("y", vec![-2, -2, 5, -2]);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_pipeline() {
        let prog = parse_from_file("examples/basic/pipeline.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 9);
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 0);
        trace.enq_scalar("en", 1);
        trace.enq_scalar("en", 1);
        trace.enq_scalar("en", 0);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 9);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_counter() {
        let prog = parse_from_file("examples/basic/counter.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("en", 1);
        trace.enq_scalar("en", 1);
        trace.enq_scalar("en", 0);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 1);
        trace.enq_scalar("y", 2);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_fsm() {
        let prog = parse_from_file("examples/basic/fsm.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("start", 0);
        trace.enq_scalar("start", 1);
        trace.enq_scalar("start", 0);
        trace.enq_scalar("start", 0);
        trace.enq_scalar("done", 0);
        trace.enq_scalar("done", 0);
        trace.enq_scalar("done", 1);
        trace.enq_scalar("done", 0);
        trace.enq_scalar("state", 0);
        trace.enq_scalar("state", 0);
        trace.enq_scalar("state", 1);
        trace.enq_scalar("state", 0);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_dot() {
        // y = bias + a*b + c*d;
        // y = 3 + -2*3 + 7*2;
        // y = 9 + 1*2 + -3*4;
        let prog = parse_from_file("examples/basic/dot.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("en", 1);
        trace.enq_scalar("en", 1);
        trace.enq_scalar("en", 1);
        trace.enq_scalar("en", 1);
        trace.enq_scalar("en", 1);
        trace.enq_scalar("en", 0);
        trace.enq_scalar("bias", 0);
        trace.enq_scalar("bias", 0);
        trace.enq_scalar("bias", 3);
        trace.enq_scalar("bias", 9);
        trace.enq_scalar("bias", 0);
        trace.enq_scalar("bias", 0);
        trace.enq_scalar("a", -2);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 0);
        trace.enq_scalar("a", 0);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("b", 2);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("c", 0);
        trace.enq_scalar("c", 7);
        trace.enq_scalar("c", -3);
        trace.enq_scalar("c", 0);
        trace.enq_scalar("c", 0);
        trace.enq_scalar("c", 0);
        trace.enq_scalar("d", 0);
        trace.enq_scalar("d", 2);
        trace.enq_scalar("d", 4);
        trace.enq_scalar("d", 0);
        trace.enq_scalar("d", 0);
        trace.enq_scalar("d", 0);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 11);
        trace.enq_scalar("y", -1);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_add_pair() {
        let prog = parse_from_file("examples/basic/add_pair.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 1);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("c", -3);
        trace.enq_scalar("d", -7);
        trace.enq_scalar("f", 4);
        trace.enq_scalar("g", -10);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_alu() {
        let prog = parse_from_file("examples/basic/alu.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("op", 0);
        trace.enq_scalar("op", 1);
        trace.enq_scalar("op", 2);
        trace.enq_scalar("op", 3);
        trace.enq_scalar("op", 4);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("a", 1);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 4);
        trace.enq_scalar("y", -2);
        trace.enq_scalar("y", 1);
        trace.enq_scalar("y", 3);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }

    #[test]
    fn test_fanout() {
        // y = a + (b * c)
        // z = d + (b * c)
        let prog = parse_from_file("examples/basic/fanout.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("en", 1);
        trace.enq_scalar("en", 0);
        trace.enq_scalar("a", 4);
        trace.enq_scalar("a", 4);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("b", 3);
        trace.enq_scalar("c", 2);
        trace.enq_scalar("c", 2);
        trace.enq_scalar("d", 1);
        trace.enq_scalar("d", 1);
        trace.enq_scalar("y", 0);
        trace.enq_scalar("y", 10);
        trace.enq_scalar("z", 0);
        trace.enq_scalar("z", 7);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .has_failed());
    }
}
