use crate::graph::graph::Graph;
use crate::graph::temp_graph::TempGraph;
use crate::graph::vertex::Vertex;
use crate::service::graph_traversal::cycle_discovery::BFSCyclehDiscovery;
use std::collections::VecDeque;

// TODO - replace TempGraph with Graph

pub fn girth<G: Graph>(graph: &G) -> usize {
    let mut shortest_cycle_length = usize::max_value();

    for vertex in graph.vertices() {
        let length_of_cycle = find_cycle(graph, vertex.index());
        if length_of_cycle < shortest_cycle_length {
            shortest_cycle_length = length_of_cycle;
        }
    }

    shortest_cycle_length
}

fn find_cycle<G: Graph>(graph: &G, vertex: usize) -> usize {
    let mut cd = BFSCyclehDiscovery::new(graph, vertex);
    cd.length_of_next_cycle()
}
