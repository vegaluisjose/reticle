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
        trace.enq("a", 3);
        trace.enq("y", 9);
        trace.enq("y", 3);
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
