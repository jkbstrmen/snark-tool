#[cfg(test)]
mod graph_tests {
    use crate::graph::graph::Graph;
    use crate::graph::undirected::simple_graph::SimpleGraph;
    use crate::graph::undirected_sparse::graph::SimpleSparseGraph;
    use crate::graph::undirected::edge::UndirectedEdge;
    use crate::graph::edge::EdgeConstructor;

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
        let mut second = SimpleGraph::with_capacity(3, 3);
        second.add_edge(2, 1);
        second.add_edge(0, 2);
        second.add_edge(0, 1);
        assert_eq!(first, second);
    }

    #[test]
    fn should_not_be_equal() {
        let first = get_graph();
        let mut second = SimpleGraph::with_capacity(3, 3);
        second.add_edge(2, 1);
        second.add_edge(0, 2);
        second.add_edge(0, 3);
        assert_ne!(first, second);
    }

    fn get_graph() -> SimpleGraph {
        let mut graph = SimpleGraph::with_capacity(3, 3);
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);
        graph.add_edge(0, 1);
        graph
    }

    #[test]
    fn simple_sparse_graph() {
        let mut graph = SimpleSparseGraph::with_capacity(10);

        graph.add_edge(0, 1);
        graph.add_edge(2, 5);
        graph.add_edge(2, 4);

        let mut edges = graph.edges();
        let edge = edges.next();
        assert_eq!(edge, Some(UndirectedEdge::new(0, 1)));

        assert_eq!(graph.has_edge(0, 1), true);
        assert_eq!(graph.has_edge(2, 4), true);
        assert_eq!(graph.has_edge(2, 5), true);
        assert_eq!(graph.has_edge(1, 4), false);
        assert_eq!(graph.has_edge(2, 6), false);

        assert_eq!(graph.size(), 6);
    }
}
