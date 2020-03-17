use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::EdgeRef;
use petgraph::Undirected;

use std::mem;

type Graph = StableGraph<u8, u16, Undirected, u8>;

pub fn get_edges_vec(graph: &Graph) -> Vec<(usize, usize)> {
    let mut vec = Vec::new();
    for node_index in graph.node_indices() {
        for edge in graph.edges(node_index) {
            let mut from = node_index.index();
            let mut to = edge.target().index();

            if from > to {
                mem::swap(&mut from, &mut to);
            }

            vec.push((from, to));
        }
    }
    vec.sort();
    vec.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
    vec
}
