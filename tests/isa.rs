use reticle::frontend::parser::parse_from_file;
use reticle::interp::trace::Trace;
use reticle::interp::Interpreter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let prog = parse_from_file("examples/isa/identity.ret");
        let mut trace = Trace::default();
        trace.enq("a", 9);
        trace.enq("y", 9);
        trace.enq("a", 3);
        trace.enq("y", 3);
        assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    }

    #[test]
    fn test_add() {
        let prog = parse_from_file("examples/isa/add.ret");
        let mut trace = Trace::default();
        trace.enq("a", 9);
        trace.enq("b", 7);
        trace.enq("y", 16);
        assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    }

    #[test]
    fn test_sub() {
        let prog = parse_from_file("examples/isa/sub.ret");
        let mut trace = Trace::default();
        trace.enq("a", 9);
        trace.enq("b", 7);
        trace.enq("y", 2);
        assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    }

    #[test]
    fn test_mul() {
        let prog = parse_from_file("examples/isa/mul.ret");
        let mut trace = Trace::default();
        trace.enq("a", 3);
        trace.enq("b", 5);
        trace.enq("y", 15);
        assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    }

    #[test]
    // FIXME: need to start checking types in order to make this work properly
    fn test_not() {
        let prog = parse_from_file("examples/isa/not.ret");
        let mut trace = Trace::default();
        trace.enq("a", 0xfffffffffffffff0u64 as i64);
        trace.enq("y", 0xfu64 as i64);
        assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    }

    #[test]
    fn test_mux() {
        let prog = parse_from_file("examples/isa/mux.ret");
        let mut trace = Trace::default();
        trace.enq("cond", 0);
        trace.enq("t", 2);
        trace.enq("f", 6);
        trace.enq("y", 6);
        trace.enq("cond", 1);
        trace.enq("t", 2);
        trace.enq("f", 6);
        trace.enq("y", 2);
        assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    }

    #[test]
    fn test_register() {
        let prog = parse_from_file("examples/isa/reg.ret");
        let mut trace = Trace::default();
        trace.enq("a", 9);
        trace.enq("en", 1);
        trace.enq("y", 3);
        trace.enq("a", 0);
        trace.enq("en", 0);
        trace.enq("y", 9);
        trace.enq("a", 0);
        trace.enq("en", 0);
        trace.enq("y", 9);
        assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    }
}
