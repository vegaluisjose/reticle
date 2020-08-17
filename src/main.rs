use reticle::frontend::parser::parse_from_file;
use reticle::passes::map::dag::Dag;
use reticle::passes::map::forest::Forest;

fn main() {
    let prog = parse_from_file("examples/basic/fsm.ret");
    println!("{}", &prog);
    let dag = Dag::from(prog);
    let forest = Forest::from(dag);
    for (id, tree) in forest.iter() {
        println!("id:{}\n{}", id, tree);
    }
}
