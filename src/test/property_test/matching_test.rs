#[cfg(test)]
pub mod matching_tests {
    use crate::graph::edge::{Edge, EdgeConstructor};
    use crate::graph::graph::{Graph, GraphConstructor};
    use crate::graph::undirected::edge::UndirectedEdge;
    use crate::graph::undirected::simple_graph::SimpleGraph;
    use crate::graph::vertex::Vertex;
    use crate::service::colour::colouriser::Colourizer;
    use crate::service::colour::matching::{CycleDiscovery, MatchingColouriser};
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::matching::perfect_matchings::{
        BfsGraph, MatchingGraph
    };
    use crate::test::test_data::test_data;

    #[test]
    fn should_traverse_using_bfs() {
        let graph = test_data::get_petersen_graph();

        let mut match_graph = MatchingGraph::new();
        for edge in graph.edges() {
            match_graph.add_edge(edge.from(), edge.to());
        }
        let bfs_vertices = vec![0, 4, 6, 8, 2, 5, 1, 7, 3, 9];
        let mut index = 0;
        let mut bfs_graph = BfsGraph::new(&match_graph, 0);
        while let Some(vertex) = bfs_graph.bfs_next() {
            assert_eq!(bfs_vertices[index], vertex);
            index += 1;
        }
    }

    #[test]
    fn should_not_have_odd_size_component() {
        let mut match_graph = MatchingGraph::new();
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, false);

        add_edges_to_graph(&mut match_graph, test_data::first_even_component());
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, false);

        let mut match_graph = MatchingGraph::new();
        add_edges_to_graph(&mut match_graph, test_data::second_even_component());
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, false);

        let mut match_graph = MatchingGraph::new();
        add_edges_to_graph(&mut match_graph, test_data::third_even_component_petersen());
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, false);

        add_edges_to_graph(&mut match_graph, test_data::second_even_component());
        add_edges_to_graph(&mut match_graph, test_data::first_even_component());
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, false);

        add_edges_to_graph(&mut match_graph, test_data::first_odd_component());
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, true);

        let mut match_graph = MatchingGraph::new();
        add_edges_to_graph(&mut match_graph, test_data::second_odd_component());
        match_graph.remove_vertex(16);
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, false);
    }

    #[test]
    fn should_have_odd_size_component() {
        let mut match_graph = MatchingGraph::new();
        add_edges_to_graph(&mut match_graph, test_data::first_odd_component());
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, true);

        add_edges_to_graph(&mut match_graph, test_data::first_even_component());
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, true);

        add_edges_to_graph(&mut match_graph, test_data::second_even_component());
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, true);

        let mut match_graph = MatchingGraph::new();
        add_edges_to_graph(&mut match_graph, test_data::first_even_component());
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, false);

        add_edges_to_graph(&mut match_graph, test_data::second_odd_component());
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, true);

        let mut match_graph = MatchingGraph::new();
        match_graph.create_vertex_if_not_exists(0);
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, true);
        match_graph.create_vertex_if_not_exists(1);
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, true);
    }

    fn add_edges_to_graph(graph: &mut MatchingGraph, edges: Vec<UndirectedEdge>) {
        for edge in edges {
            graph.add_edge(edge.from(), edge.to());
        }
    }

    #[test]
    fn should_have_odd_cycle() {

        // TODO
    }

    #[test]
    fn should_have_even_cycle() {

        // TODO
    }

    #[test]
    fn should_find_all_perfect_matchings() {
        // let mut graph = MatchingGraph::from_graph(&test_data::get_petersen_graph());
        let mut graph = MatchingGraph::from_graph(&test_data::get_petersen_graph());
        let mut matchings = graph.perfect_matchings();
        let petersens_matchings = test_data::petersens_matchings();

        for mut matching in matchings.iter_mut() {
            matching.edges.sort();
        }
        matchings.sort();

        assert_eq!(petersens_matchings[0], matchings[0]);
        assert_eq!(petersens_matchings[1], matchings[1]);
        assert_eq!(petersens_matchings[2], matchings[2]);
        assert_eq!(petersens_matchings[3], matchings[3]);
        assert_eq!(petersens_matchings[4], matchings[4]);
        assert_eq!(petersens_matchings[5], matchings[5]);
    }
}
