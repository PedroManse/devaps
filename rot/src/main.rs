use rot::{
    graph::Graph,
    parse,
    RotError
};
use std::fs;

//line.rot
///A
///B
///C
///A->C

fn main() -> Result<(), RotError> {
    let code = fs::read_to_string("graphs/line.rot").unwrap();
    let p = parse::parse(code)?;
    println!("{p:#?}");
    //let mut graph = Graph::default();
    //graph.new_node("A", HashMap::new());
    //graph.new_node("B", HashMap::new());
    //graph.new_node("C", HashMap::new());
    //let aid = graph.get_id_by_name("A")?;
    //let bid = graph.get_id_by_name("C")?;
    //graph.link_nodes(
    //    aid,
    //    bid,
    //    HashMap::from([("text".to_owned(), "hello".to_owned())]),
    //)?;

    //let dot_graph = rot::export::to::rot(&graph);
    // $rot cyclic.rot --ret dot
    //println!("{dot_graph:#}");
    Ok(())
}
