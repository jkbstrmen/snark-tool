use crate::graph::edge::EdgeConstructor;
use crate::graph::graph::{Graph, GraphConstructor};
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::simple_edge_graph::graph::SimpleEdgeGraph;

#[test]
fn should_create_graph() {
    let graph = get_graph();
    assert_eq!(graph.has_edge(0, 1), true);
    assert_eq!(graph.has_edge(0, 2), true);
    assert_eq!(graph.has_edge(1, 2), true);
    assert_eq!(graph.has_edge(2, 1), true);
    assert_eq!(graph.has_edge(3, 1), false);
    assert_eq!(graph.has_edge(1, 1), false);
    assert_eq!(graph.has_edge(1, 3), false);
}

#[test]
fn should_format_graph() {
    let graph = get_graph();
    let graph = format!("{}", graph);
    let string = "0: 1, 2\n1: 0, 2\n2: 0, 1\n";
    assert_eq!(graph, string);
}

#[test]
fn should_be_equal() {
    let first = get_graph();
    let mut second = SimpleEdgeGraph::with_capacity(3, 3);
    second.add_edge(2, 1);
    second.add_edge(0, 2);
    second.add_edge(0, 1);
    assert_eq!(first, second);
}

#[test]
fn should_not_be_equal() {
    let first = get_graph();
    let mut second = SimpleEdgeGraph::with_capacity(3, 3);
    second.add_edge(2, 1);
    second.add_edge(0, 2);
    second.add_edge(0, 3);
    assert_ne!(first, second);
}

fn get_graph() -> SimpleEdgeGraph {
    let mut graph = SimpleEdgeGraph::with_capacity(3, 3);
    graph.add_vertex();
    graph.add_vertex();
    graph.add_vertex();

    graph.add_edge(2, 0);
    graph.add_edge(2, 1);
    graph.add_edge(0, 1);
    graph
}

#[test]
fn should_iter_edges() {
    let graph = get_graph();
    let mut index = 0;
    for edge in graph.edges() {
        if index == 0 {
            assert_eq!(edge, &UndirectedEdge::new(0, 1));
        }
        if index == 1 {
            assert_eq!(edge, &UndirectedEdge::new(0, 2));
        }
        if index == 2 {
            assert_eq!(edge, &UndirectedEdge::new(1, 2));
        }
        index += 1;
    }
    assert_eq!(index, 3);
}

#[test]
fn should_iter_edges_of_vertex() {
    let graph = get_graph();
    let mut index = 0;
    for edge in graph.edges_of_vertex(1) {
        if index == 0 {
            assert_eq!(edge, &UndirectedEdge::new(0, 1));
        }
        if index == 1 {
            assert_eq!(edge, &UndirectedEdge::new(1, 2));
        }
        index += 1;
    }
    assert_eq!(index, 2);
}

#[test]
fn should_remove_edges_of_vertex() {
    let mut graph = get_graph();
    assert_eq!(graph.has_edge(0, 1), true);
    assert_eq!(graph.has_edge(0, 2), true);
    assert_eq!(graph.has_edge(1, 2), true);
    graph.remove_edge(0, 1);
    assert_eq!(graph.has_edge(0, 1), false);
    assert_eq!(graph.has_edge(0, 2), true);
    graph.remove_edges_of_vertex(0);
    assert_eq!(graph.has_edge(0, 1), false);
    assert_eq!(graph.has_edge(0, 2), false);
    assert_eq!(graph.has_edge(1, 2), true);
}
