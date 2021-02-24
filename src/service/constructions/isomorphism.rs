use crate::graph::graph::Graph;
use petgraph::graph::{UnGraph, NodeIndex};
use crate::graph::vertex::Vertex;

pub fn is_isomorphic<G: Graph>(first: &G, second: &G) -> bool {
    let first_petgraph = to_petgraph(first);
    let second_petgraph = to_petgraph(second);
    petgraph::algo::is_isomorphic(&first_petgraph, &second_petgraph)

    // if first_petgraph.contains_edge(NodeIndex::new(0), NodeIndex::new(1)) &&
    //     second_petgraph.contains_edge(NodeIndex::new(0), NodeIndex::new(1))
    // {
    //     return false;
    // }
    // true
}

fn to_petgraph<G: Graph>(graph: &G) -> UnGraph<usize, usize>{
    let mut petgraph = UnGraph::new_undirected();
    for _vertex in graph.vertices() {
        petgraph.add_node(0);
    }
    for vertex in graph.vertices() {
        for neighbor in graph.neighbors_of_vertex(vertex.index()) {
            petgraph.add_edge(NodeIndex::new(vertex.index()), NodeIndex::new(neighbor), 0);
        }
    }
    petgraph
}