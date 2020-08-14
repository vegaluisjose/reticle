use reticle::frontend::parser::parse_from_file;
use reticle::interp::trace::Trace;
use reticle::interp::ty::Value;
use reticle::interp::Interpreter;

#[cfg(test)]
mod test_basic {
    use super::*;

    #[test]
    fn test_vadd_one() {
        let prog = parse_from_file("examples/basic/vadd_one.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::from(vec![-4, 2, 0, 1]));
        trace.enq("y", Value::from(vec![-3, 3, 1, 2]));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_pipeline() {
        let prog = parse_from_file("examples/basic/pipeline.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(9));
        trace.enq("a", Value::new_scalar(0));
        trace.enq("a", Value::new_scalar(0));
        trace.enq("en", Value::new_scalar(1));
        trace.enq("en", Value::new_scalar(1));
        trace.enq("en", Value::new_scalar(0));
        trace.enq("y", Value::new_scalar(0));
        trace.enq("y", Value::new_scalar(0));
        trace.enq("y", Value::new_scalar(9));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_muladd() {
        let prog = parse_from_file("examples/basic/muladd.ret");
        let mut trace = Trace::default();
        trace.enq("a", Value::new_scalar(4));
        trace.enq("a", Value::new_scalar(0));
        trace.enq("b", Value::new_scalar(2));
        trace.enq("b", Value::new_scalar(0));
        trace.enq("c", Value::new_scalar(3));
        trace.enq("c", Value::new_scalar(3));
        trace.enq("en", Value::new_scalar(1));
        trace.enq("en", Value::new_scalar(0));
        trace.enq("y", Value::new_scalar(3));
        trace.enq("y", Value::new_scalar(11));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_counter() {
        let prog = parse_from_file("examples/basic/counter.ret");
        let mut trace = Trace::default();
        trace.enq("en", Value::new_scalar(1));
        trace.enq("en", Value::new_scalar(1));
        trace.enq("en", Value::new_scalar(0));
        trace.enq("y", Value::new_scalar(0));
        trace.enq("y", Value::new_scalar(1));
        trace.enq("y", Value::new_scalar(2));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }

    #[test]
    fn test_fsm_two() {
        let prog = parse_from_file("examples/basic/fsm_two.ret");
        let mut trace = Trace::default();
        trace.enq("start", Value::new_scalar(0));
        trace.enq("start", Value::new_scalar(1));
        trace.enq("start", Value::new_scalar(0));
        trace.enq("start", Value::new_scalar(0));
        trace.enq("done", Value::new_scalar(0));
        trace.enq("done", Value::new_scalar(0));
        trace.enq("done", Value::new_scalar(1));
        trace.enq("done", Value::new_scalar(0));
        trace.enq("state", Value::new_scalar(0));
        trace.enq("state", Value::new_scalar(0));
        trace.enq("state", Value::new_scalar(1));
        trace.enq("state", Value::new_scalar(0));
        assert!(!Interpreter::default()
            .with_print()
            .run(&prog, &trace)
            .is_failed());
    }
}
