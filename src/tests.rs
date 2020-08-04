#[cfg(test)]
mod tests {
    use crate::frontend::parser::parse_from_file;
    use crate::lang::interp::trace::Trace;
    use crate::lang::interp::Interpreter;
    #[test]
    fn test_identity() {
        let prog = parse_from_file("examples/identity.ret");
        let mut trace = Trace::default();
        trace.enq("a", 9);
        trace.enq("a", 3);
        trace.enq("y", 9);
        trace.enq("y", 3);
        assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    }

    #[test]
    fn test_register() {
        let prog = parse_from_file("examples/reg.ret");
        let mut trace = Trace::default();
        trace.enq("a", 9);
        trace.enq("en", 1);
        trace.enq("y", 0);
        trace.enq("a", 0);
        trace.enq("en", 0);
        trace.enq("y", 9);
        trace.enq("a", 0);
        trace.enq("en", 0);
        trace.enq("y", 9);
        assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    }

    #[test]
    fn test_pipeline() {
        let prog = parse_from_file("examples/pipeline.ret");
        let mut trace = Trace::default();
        trace.enq("a", 9);
        trace.enq("en", 1);
        trace.enq("y", 0);
        trace.enq("a", 0);
        trace.enq("en", 1);
        trace.enq("y", 0);
        trace.enq("a", 0);
        trace.enq("en", 0);
        trace.enq("y", 9);
        assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    }
}
