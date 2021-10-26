use snark_tool::graph::graph::{Graph, GraphConstructor};
use snark_tool::graph::undirected::simple_graph::graph::SimpleGraph;

fn main() {
    let mut graph = SimpleGraph::new();
    graph.add_edge(0, 5);
    graph.add_edge(2, 4);

    for edge in graph.edges() {
        println!("{:?}", edge);
    }
}
