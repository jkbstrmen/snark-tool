use crate::graph::graph::{Edge, Graph, SimpleGraph, UndirectedEdge, UndirectedEdges};

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
    let edges = vec![
        UndirectedEdge::new(0, 1),
        UndirectedEdge::new(1, 2),
        UndirectedEdge::new(2, 3),
        UndirectedEdge::new(0, 3),
    ];

    let vert = 0;
    println!("for: {}", vert);
    let mut edges_for_vertex = UndirectedEdges::new(vert, edges.iter());
    println!("{:?}", edges_for_vertex.next());
    println!("{:?}", edges_for_vertex.next());

    let vert = 1;
    println!("for: {}", vert);
    let mut edges_for_vertex = UndirectedEdges::new(vert, edges.iter());
    println!("{:?}", edges_for_vertex.next());
    println!("{:?}", edges_for_vertex.next());

    let vert = 2;
    println!("for: {}", vert);
    let mut edges_for_vertex = UndirectedEdges::new(vert, edges.iter());
    println!("{:?}", edges_for_vertex.next());
    println!("{:?}", edges_for_vertex.next());
}
