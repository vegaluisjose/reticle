use reticle::frontend::parser::parse_from_file;
use reticle::lang::interp::trace::Trace;
use reticle::lang::interp::Interpreter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline() {
        let prog = parse_from_file("examples/basic/pipeline.ret");
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

    #[test]
    fn test_muladd() {
        let prog = parse_from_file("examples/basic/muladd.ret");
        let mut trace = Trace::default();
        trace.enq("a", 4);
        trace.enq("b", 2);
        trace.enq("c", 3);
        trace.enq("en", 1);
        trace.enq("y", 3);
        trace.enq("a", 0);
        trace.enq("b", 0);
        trace.enq("c", 3);
        trace.enq("en", 0);
        trace.enq("y", 11);
        assert!(!Interpreter::default().run(&prog, &trace).is_failed());
    }
}
