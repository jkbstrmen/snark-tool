use crate::graph::traits::graph::Graph;
use crate::graph::undirected::simple_graph::SimpleGraph;
use std::collections::HashMap;
use std::iter::FromIterator;

#[test]
fn should_create_graph() {
    let mut graph = SimpleGraph::with_capacity(3, 3);
    graph.add_vertex();
    graph.add_vertex();
    graph.add_vertex();

    graph.add_edge(2, 0);
    graph.add_edge(2, 1);
    graph.add_edge(0, 1);

    println!("{}", graph);
    println!("{:#?}", graph);

    let str = String::from("ola");
    str.chars();
}

#[test]
fn should_iter_edges() {
    // let edges = vec![
    //     UndirectedEdge::new(0, 1),
    //     UndirectedEdge::new(1, 2),
    //     UndirectedEdge::new(2, 3),
    //     UndirectedEdge::new(0, 3),
    // ];
    //
    // let vert = 0;
    // println!("for: {}", vert);
    // let mut edges_for_vertex = UndirectedEdges::new(vert, edges.iter());
    // println!("{:?}", edges_for_vertex.next());
    // println!("{:?}", edges_for_vertex.next());
    //
    // let vert = 1;
    // println!("for: {}", vert);
    // let mut edges_for_vertex = UndirectedEdges::new(vert, edges.iter());
    // println!("{:?}", edges_for_vertex.next());
    // println!("{:?}", edges_for_vertex.next());
    //
    // let vert = 2;
    // println!("for: {}", vert);
    // let mut edges_for_vertex = UndirectedEdges::new(vert, edges.iter());
    // println!("{:?}", edges_for_vertex.next());
    // println!("{:?}", edges_for_vertex.next());
}

#[test]
fn should_iter_edges_2() {
    let mut graph = SimpleGraph::with_capacity(5, 5);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);

    // let mut edges = graph.edges_of_vertex(0);
    // println!("{:?}", edges.next());
    // println!("{:?}", edges.next());
    //
    // let mut edges = graph.edges_of_vertex(1);
    // println!("{:?}", edges.next());
    // println!("{:?}", edges.next());

    println!("{}", graph);

    // let mut edges = graph.edges_of_vertex(0);
    // let vert = Vec::from_iter(edges);
    // println!("{:?}", vert);
}

// #[derive(Debug)]
// struct Hell {
//     hell: HashMap<u32, u32>,
//     edges: HashMap<SimpleVertex, Vec<UndirectedEdge>>
// }
//
// #[test]
// fn test() {
//
//     let mut hell = Hell{
//         hell: HashMap::new(),
//         edges: HashMap::new()
//     };
//     hell.hell.insert(1, 2);
//     // hell.edges.insert(SimpleVertex::new(5));
//
//
//     println!("{:?}", hell);
//
// }
