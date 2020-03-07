use crate::service::io::reader;
use crate::service::io::reader::read_graph;

#[test]
fn it_works() {

    // rea

    let graph_path = "resources/graphs/graphG6.g6";
    let content = std::fs::read_to_string(graph_path).expect("could not read file");
    // println!("Graph file content: {}", content);

    read_graph(content.as_str());

    // println!("Testing");
    // assert_eq!(2 + 2, 4);
}