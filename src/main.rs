use reticle::frontend::parser::parse_from_file;
use reticle::passes::map::dag::Dag;
use reticle::passes::map::partition::Partition;

fn main() {
    let prog = parse_from_file("examples/basic/fsm.ret");
    let dag = Dag::from(prog.clone());
    let part = Partition::from(dag.clone());
    for (id, tree) in part.iter() {
        println!("id:{}\n{}", id, tree);
    }
    println!("{}", prog);
    println!("{}", dag);
}
