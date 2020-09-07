#[cfg(test)]
mod graph_tests {
    use crate::graph::edge::EdgeConstructor;
    use crate::graph::graph::{Graph, GraphConstructor};
    use crate::graph::undirected::edge::UndirectedEdge;
    use crate::graph::undirected::simple_graph::SimpleGraph;
    use crate::graph::undirected_sparse::graph::SimpleSparseGraph;
    use crate::service::matching::perfect_matchings::{MatchingGraph, Vertex};
    use crate::test::test_data::test_data;

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

    fn get_ss_graph() -> SimpleSparseGraph {
        let mut graph = SimpleSparseGraph::with_capacity(3, 3);
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);
        graph.add_edge(0, 1);
        graph
    }

    #[test]
    fn should_iter_edges() {
        let graph = get_graph();
        let mut index = 0;
        for edge in graph.edges() {
            if index == 0 {
                assert_eq!(edge, &UndirectedEdge::new(0, 1));
            }
            if index == 1 {
                assert_eq!(edge, &UndirectedEdge::new(0, 2));
            }
            if index == 2 {
                assert_eq!(edge, &UndirectedEdge::new(1, 2));
            }
            index += 1;
        }
        assert_eq!(index, 3);
    }

    #[test]
    fn should_iter_edges_of_vertex() {
        let graph = get_graph();
        let mut index = 0;
        for edge in graph.edges_of_vertex(1) {
            if index == 0 {
                assert_eq!(edge, &UndirectedEdge::new(0, 1));
            }
            if index == 1 {
                assert_eq!(edge, &UndirectedEdge::new(1, 2));
            }
            index += 1;
        }
        assert_eq!(index, 2);
    }

    #[test]
    fn should_remove_edges_of_vertex() {
        let mut graph = get_graph();
        assert_eq!(graph.has_edge(0, 1), true);
        assert_eq!(graph.has_edge(0, 2), true);
        assert_eq!(graph.has_edge(1, 2), true);
        graph.remove_edge(0, 1);
        assert_eq!(graph.has_edge(0, 1), false);
        assert_eq!(graph.has_edge(0, 2), true);
        graph.remove_edges_of_vertex(0);
        assert_eq!(graph.has_edge(0, 1), false);
        assert_eq!(graph.has_edge(0, 2), false);
        assert_eq!(graph.has_edge(1, 2), true);
    }

    #[test]
    fn simple_sparse_graph() {
        let mut graph = SimpleSparseGraph::with_vertices_capacity(10);

        graph.add_edge(0, 1);
        graph.add_edge(2, 5);
        graph.add_edge(2, 4);

        let mut edges = graph.edges();
        let _edge = edges.next();
        // assert_eq!(edge, Some(UndirectedEdge::new(0, 1)));

        assert_eq!(graph.has_edge(0, 1), true);
        assert_eq!(graph.has_edge(2, 4), true);
        assert_eq!(graph.has_edge(2, 5), true);
        assert_eq!(graph.has_edge(1, 4), false);
        assert_eq!(graph.has_edge(2, 6), false);

        assert_eq!(graph.size(), 6);
    }

    #[test]
    fn should_iter_edges_ssg() {
        let graph = get_ss_graph();
        let mut edges = vec![];
        edges.push(UndirectedEdge::new(0, 1));
        edges.push(UndirectedEdge::new(0, 2));
        edges.push(UndirectedEdge::new(1, 2));

        let mut index = 0;
        for edge in graph.edges() {
            edges.retain(|edge_temp| edge_temp != edge);
            index += 1;
        }
        assert_eq!(index, 3);
        assert_eq!(edges.len(), 0);
    }

    #[test]
    fn should_iter_edges_of_vertex_ssg() {
        let graph = get_ss_graph();
        let mut edges = vec![];
        edges.push(UndirectedEdge::new(0, 1));
        edges.push(UndirectedEdge::new(1, 2));
        let mut index = 0;
        for edge in graph.edges_of_vertex(1) {
            edges.retain(|edge_temp| edge_temp != edge);
            index += 1;
        }
        assert_eq!(index, 2);
    }

    #[test]
    fn should_remove_edges_of_vertex_ssg() {
        let mut graph = get_ss_graph();
        assert_eq!(graph.vertices[0].edges.len(), 2);
        assert_eq!(graph.vertices[1].edges.len(), 2);
        assert_eq!(graph.vertices[2].edges.len(), 2);
        graph.remove_edges_of_vertex(0);
        assert_eq!(graph.vertices[0].edges.len(), 0);
        assert_eq!(graph.vertices[1].edges.len(), 1);
        assert_eq!(graph.vertices[2].edges.len(), 1);
    }

    //
    // Matching graph
    //

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
        graph.add_vertex(Vertex::new(0));
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
