#[cfg(test)]
pub mod test_data {
    use crate::graph::traits::graph::Graph;
    use crate::graph::undirected::simple_graph::SimpleGraph;

    pub fn get_petersen_graph() -> SimpleGraph {
        let mut graph = SimpleGraph::with_capacity(10, 15);
        graph.add_edge(0, 4);
        graph.add_edge(0, 6);
        graph.add_edge(0, 8);
        graph.add_edge(1, 5);
        graph.add_edge(1, 6);
        graph.add_edge(1, 9);
        graph.add_edge(2, 4);
        graph.add_edge(2, 7);
        graph.add_edge(2, 9);
        graph.add_edge(3, 5);
        graph.add_edge(3, 7);
        graph.add_edge(3, 8);
        graph.add_edge(4, 5);
        graph.add_edge(6, 7);
        graph.add_edge(8, 9);
        graph
    }
}
