use crate::graph::graph::{Graph, GraphConstructor};
use crate::graph::undirected::multi_graph::graph::MultiGraph;
use crate::service::property::max_flow::max_flow::FordFulkerson;

#[test]
fn should_have_max_flow_two() {
    let mut graph = MultiGraph::new();
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(3, 4);
    graph.add_edge(3, 5);
    graph.add_edge(4, 5);

    let max_flow = FordFulkerson::max_flow(&graph, 0, 5);
    assert_eq!(max_flow, 2);
}
