#[cfg(test)]
mod residual_graph_tests {
    use crate::graph::edge::EdgeConstructor;
    use crate::graph::graph::{Graph, GraphConstructor};
    use crate::graph::vertex::Vertex;
    use crate::service::property::max_flow::residual_graph::edge::DirectedFlowEdge;
    use crate::service::property::max_flow::residual_graph::graph::ResidualGraph;

    fn get_graph() -> ResidualGraph {
        let mut graph = ResidualGraph::with_capacity(3, 3);
        graph.add_vertex();
        graph.add_vertex();
        graph.add_vertex();

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);
        graph.add_edge(0, 1);
        graph
    }

    #[test]
    fn residual_graph() {
        let mut graph = ResidualGraph::with_vertices_capacity(10);

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
    fn should_iter_edges() {
        let graph = get_graph();
        let mut edges = vec![];
        edges.push(DirectedFlowEdge::new_with_capacity(0, 1, 1));
        edges.push(DirectedFlowEdge::new_with_capacity(2, 0, 1));
        edges.push(DirectedFlowEdge::new_with_capacity(2, 1, 1));

        let mut index = 0;
        for edge in graph.edges() {
            edges.retain(|edge_temp| edge_temp != edge);
            index += 1;
        }
        assert_eq!(index, 3);
        assert_eq!(edges.len(), 0);
    }

    #[test]
    fn should_iter_edges_of_vertex() {
        let graph = get_graph();
        let mut edges = vec![];
        edges.push(DirectedFlowEdge::new(0, 1));
        edges.push(DirectedFlowEdge::new(1, 2));
        let mut index = 0;
        for edge in graph.edges_of_vertex(1) {
            edges.retain(|edge_temp| edge_temp != edge);
            index += 1;
        }
        assert_eq!(index, 0);
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn should_remove_edges_of_vertex() {
        let mut graph = get_graph();
        assert_eq!(graph.vertices[0].edges.len(), 1);
        assert_eq!(graph.vertices[1].edges.len(), 0);
        assert_eq!(graph.vertices[2].edges.len(), 2);
        graph.remove_edges_of_vertex(0);
        assert_eq!(graph.vertices[0].edges.len(), 0);
        assert_eq!(graph.vertices[1].edges.len(), 0);
        assert_eq!(graph.vertices[2].edges.len(), 2);
    }

    #[test]
    fn should_have_first_vertex() {
        let mut graph = get_graph();
        let first_vertex = graph.first_vertex();
        assert_eq!(first_vertex.is_some(), true);
        assert_eq!(first_vertex.unwrap().index(), 0);

        let mut graph = ResidualGraph::new();
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
    fn should_add_vertex_with_index() {
        let mut graph = ResidualGraph::new();
        graph.add_vertex_with_index(1);
        assert_eq!(graph.size(), 2);
        graph.add_vertex();
        assert_eq!(graph.size(), 3);
        assert_eq!(graph.has_vertex(0), false);
        assert_eq!(graph.has_vertex(1), true);
        graph.add_vertex_with_index(2);
        assert_eq!(graph.size(), 3);
        graph.add_vertex_with_index(3);
        assert_eq!(graph.size(), 4);
        graph.add_vertex_with_index(5);
        assert_eq!(graph.size(), 6);
        assert_eq!(graph.has_vertex(4), false);
        assert_eq!(graph.has_vertex(5), true);
    }

    #[test]
    fn should_increase_decrease_capacity() {
        let mut graph = get_graph();

        assert_eq!(graph.edge_capacity(2, 0), 1);
        graph.increase_edge_capacity(2, 0);
        assert_eq!(graph.edge_capacity(2, 0), 2);

        let decreased = graph.decrease_edge_capacity(2, 0);
        assert_eq!(decreased, true);
        let decreased = graph.decrease_edge_capacity(2, 0);
        assert_eq!(decreased, true);
        let decreased = graph.decrease_edge_capacity(2, 0);
        assert_eq!(decreased, false);
        assert_eq!(graph.edge_capacity(2, 0), 0);

        graph.add_edge(2, 0);
        assert_eq!(graph.edge_capacity(2, 0), 1);
    }
}
