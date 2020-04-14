#[cfg(test)]
mod graph_tests {
    use crate::graph::traits::graph::Graph;
    use crate::graph::undirected::simple_graph::SimpleGraph;

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
}
