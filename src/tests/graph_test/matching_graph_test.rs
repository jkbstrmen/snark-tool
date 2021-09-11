#[cfg(test)]
mod matching_graph_tests {
    use crate::service::matching::perfect_matchings::{MatchingGraph, MatchingVertex};
    use crate::tests::test_data::test_data;

    #[test]
    fn should_create_from_graph_mg() {
        let s_graph = test_data::get_petersen_graph();
        let m_graph = MatchingGraph::from_graph(&s_graph);

        assert_edges_petersen(&m_graph);
    }

    fn assert_edges_petersen(graph: &MatchingGraph) {
        assert_eq!(graph.has_edge(0, 4), true);
        assert_eq!(graph.has_edge(0, 6), true);
        assert_eq!(graph.has_edge(0, 8), true);
        assert_eq!(graph.has_edge(1, 5), true);
        assert_eq!(graph.has_edge(1, 6), true);
        assert_eq!(graph.has_edge(1, 9), true);
        assert_eq!(graph.has_edge(2, 4), true);
        assert_eq!(graph.has_edge(2, 7), true);
        assert_eq!(graph.has_edge(2, 9), true);
        assert_eq!(graph.has_edge(3, 5), true);
        assert_eq!(graph.has_edge(3, 7), true);
        assert_eq!(graph.has_edge(3, 8), true);
        assert_eq!(graph.has_edge(4, 5), true);
        assert_eq!(graph.has_edge(6, 7), true);
        assert_eq!(graph.has_edge(8, 9), true);

        assert_eq!(graph.has_edge(0, 5), false);
        assert_eq!(graph.has_edge(2, 8), false);
    }

    #[test]
    fn should_create_and_modify_graph_mg() {
        let mut graph = MatchingGraph::with_capacity(10);
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

        assert_edges_petersen(&graph);

        assert_eq!(graph.size(), 10);
        graph.remove_vertex(0);
        assert_eq!(graph.has_edge(0, 4), false);
        assert_eq!(graph.has_edge(0, 6), false);
        assert_eq!(graph.has_edge(0, 8), false);
        assert_eq!(graph.size(), 9);
        graph.add_vertex(MatchingVertex::new(0));
        assert_eq!(graph.has_edge(0, 4), false);
        assert_eq!(graph.has_edge(0, 6), false);
        assert_eq!(graph.has_edge(0, 8), false);
        assert_eq!(graph.size(), 10);
        graph.add_edge(0, 4);
        graph.add_edge(0, 6);
        graph.add_edge(0, 8);

        assert_edges_petersen(&graph);
        asser_graph_simpleness(&graph);

        graph.remove_vertex(0);
        graph.remove_vertex(1);

        graph.add_edge(0, 4);
        graph.add_edge(0, 6);
        graph.add_edge(0, 8);
        graph.add_edge(1, 5);
        graph.add_edge(1, 6);
        graph.add_edge(1, 9);

        asser_graph_simpleness(&graph);
        assert_edges_petersen(&graph);
    }

    fn asser_graph_simpleness(graph: &MatchingGraph) {
        for vertex in graph.vertices() {
            let mut neighbors = vec![];
            for neighbor in vertex.neighbors() {
                neighbors.push(neighbor);
            }
            let len_before = neighbors.len();
            neighbors.sort();
            neighbors.dedup();
            assert_eq!(len_before, neighbors.len());
        }
    }

    #[test]
    fn should_add_and_remove_edges_mg() {
        let s_graph = test_data::get_petersen_graph();
        let mut m_graph = MatchingGraph::from_graph(&s_graph);

        assert_edges_petersen(&m_graph);

        m_graph.remove_edge(0, 4);
        assert_eq!(m_graph.has_edge(0, 4), false);
        m_graph.add_edge(0, 4);
        assert_eq!(m_graph.has_edge(0, 4), true);

        m_graph.remove_edge(1, 6);
        assert_eq!(m_graph.has_edge(1, 6), false);
        m_graph.add_edge(1, 6);
        assert_eq!(m_graph.has_edge(1, 6), true);

        m_graph.add_edge(1, 6);
        asser_graph_simpleness(&m_graph);
    }
}
