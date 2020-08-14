use reticle::frontend::parser::parse_from_file;
use reticle::interp::trace::Trace;
use reticle::interp::Interpreter;

#[cfg(test)]
mod test_basic {
    use super::*;

    #[test]
    fn test_vadd_one() {
        let prog = parse_from_file("examples/basic/vadd_one.ret");
        let mut trace = Trace::default();
        trace.enq_vector("a", vec![-4, 2, 0, 1]);
        trace.enq_vector("y", vec![-3, 3, 1, 2]);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
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
            .is_failed());
    }

    #[test]
    fn test_muladd() {
        let prog = parse_from_file("examples/basic/muladd.ret");
        let mut trace = Trace::default();
        trace.enq_scalar("a", 4);
        trace.enq_scalar("a", 0);
        trace.enq_scalar("b", 2);
        trace.enq_scalar("b", 0);
        trace.enq_scalar("c", 3);
        trace.enq_scalar("c", 3);
        trace.enq_scalar("en", 1);
        trace.enq_scalar("en", 0);
        trace.enq_scalar("y", 3);
        trace.enq_scalar("y", 11);
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
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
            .is_failed());
    }

    #[test]
    fn test_fsm_two() {
        let prog = parse_from_file("examples/basic/fsm_two.ret");
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
            .is_failed());
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
            .is_failed());
    }
}
