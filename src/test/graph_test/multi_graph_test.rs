#[cfg(test)]
mod multi_graph_tests {
    use crate::graph::edge::EdgeConstructor;
    use crate::graph::graph::{Graph, GraphConstructor};
    use crate::graph::multi::graph::MultiGraph;
    use crate::graph::undirected::edge::UndirectedEdge;
    use crate::graph::undirected_sparse::graph::SimpleSparseGraph;
    use crate::graph::vertex::Vertex;

    fn get_graph() -> MultiGraph {
        let mut graph = MultiGraph::with_capacity(3, 3);
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);
        graph.add_edge(0, 1);
        graph.add_edge(0, 1);
        graph
    }

    #[test]
    fn multi_graph() {
        let mut graph = SimpleSparseGraph::with_vertices_capacity(10);

        graph.add_edge(0, 1);
        graph.add_edge(2, 5);
        graph.add_edge(2, 4);
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
    fn should_iter_edges() {
        let graph = get_graph();
        let mut edges = vec![];
        edges.push(UndirectedEdge::new(0, 1));
        edges.push(UndirectedEdge::new(0, 2));
        edges.push(UndirectedEdge::new(1, 2));

        let mut index = 0;
        for edge in graph.edges() {
            edges.retain(|edge_temp| edge_temp != edge);
            index += 1;
        }
        assert_eq!(index, 4);
        assert_eq!(edges.len(), 0);
    }

    #[test]
    fn should_iter_edges_of_vertex() {
        let graph = get_graph();
        let mut edges = vec![];
        edges.push(UndirectedEdge::new(0, 1));
        edges.push(UndirectedEdge::new(1, 2));
        let mut index = 0;
        for edge in graph.edges_of_vertex(1) {
            edges.retain(|edge_temp| edge_temp != edge);
            index += 1;
        }
        assert_eq!(index, 3);
        assert_eq!(edges.len(), 0);
    }

    #[test]
    fn should_remove_edges_of_vertex() {
        let mut graph = get_graph();
        assert_eq!(graph.vertices[0].edges.len(), 3);
        assert_eq!(graph.vertices[1].edges.len(), 3);
        assert_eq!(graph.vertices[2].edges.len(), 2);
        graph.remove_edges_of_vertex(0);
        assert_eq!(graph.vertices[0].edges.len(), 0);
        assert_eq!(graph.vertices[1].edges.len(), 1);
        assert_eq!(graph.vertices[2].edges.len(), 1);
    }

    #[test]
    fn should_have_first_vertex() {
        let mut graph = get_graph();
        let first_vertex = graph.first_vertex();
        assert_eq!(first_vertex.is_some(), true);
        assert_eq!(first_vertex.unwrap().index(), 0);

        let mut graph = SimpleSparseGraph::new();
        let first_vertex = graph.first_vertex();
        assert_eq!(first_vertex.is_none(), true);

        graph.add_vertex();
        let first_vertex = graph.first_vertex();
        assert_eq!(first_vertex.is_some(), true);
        assert_eq!(first_vertex.unwrap().index(), 0);
        graph.remove_vertex(0);
        let first_vertex = graph.first_vertex();
        assert_eq!(first_vertex.is_none(), true);

        graph.add_vertex();
        let first_vertex = graph.first_vertex();
        assert_eq!(first_vertex.is_some(), true);
        assert_eq!(first_vertex.unwrap().index(), 1);
    }

    #[test]
    fn should_have_multiple_same_edges() {
        let graph = get_graph();
        let edge_check = UndirectedEdge::new(0, 1);
        let mut counter = 0;
        for edge in graph.edges() {
            if edge.eq(&edge_check) {
                counter += 1;
            }
        }
        assert_eq!(counter, 2);

        let edge_check = UndirectedEdge::new(0, 2);
        let mut counter = 0;
        for edge in graph.edges() {
            if edge.eq(&edge_check) {
                counter += 1;
            }
        }
        assert_eq!(counter, 1);
    }
}
