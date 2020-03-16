use petgraph::stable_graph::StableGraph;
use petgraph::Undirected;

use std::io::Write;

type Graph = StableGraph<u8, u16, Undirected, u8>;

pub fn write_graph(graph: Graph, buffer: &mut impl Write) {
    // let graph_string = to_g6_string(graph);
    // writeln!(buffer, "{}", graph_string);
}
