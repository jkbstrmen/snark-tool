use crate::graph::graph::Graph;
use crate::graph::temp_graph::TempGraph;
use crate::graph::vertex::Vertex;
use std::collections::VecDeque;

// TODO - replace TempGraph with Graph

pub fn girth<G: TempGraph>(graph: &G) -> usize {
    let mut shortest_cycle_length = usize::max_value();

    for vertex in graph.vertices() {
        let length_of_cycle = find_cycle(graph, vertex.index());
        if length_of_cycle < shortest_cycle_length {
            shortest_cycle_length = length_of_cycle;
        }
    }

    shortest_cycle_length
}

fn find_cycle<G: TempGraph>(graph: &G, vertex: usize) -> usize {
    //  TODO - use BFS to find cycle

    0
}
