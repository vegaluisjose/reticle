use reticle::frontend::parser::parse_from_file;
use reticle::passes::map::dag::Dag;
use reticle::passes::map::forest::Forest;

fn main() {
    let prog = parse_from_file("examples/basic/fsm.ret");
    let dag = Dag::from(prog.clone());
    let forest = Forest::from(dag.clone());
    for (id, tree) in forest.iter() {
        println!("id:{}\n{}", id, tree);
    }
    println!("{}", prog);
    println!("{}", dag);
}
