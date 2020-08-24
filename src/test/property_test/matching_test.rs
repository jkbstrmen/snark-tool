#[cfg(test)]
pub mod matching_tests {
    use crate::graph::edge::{Edge, EdgeConstructor};
    use crate::graph::graph::{Graph, GraphConstructor};
    use crate::graph::undirected::simple_graph::SimpleGraph;
    use crate::graph::vertex::Vertex;
    use crate::service::colour::colouriser::Colourizer;
    use crate::service::colour::matching::MatchingColouriser;
    use crate::service::io::reader_g6::G6Reader;
    use crate::test::test_data::test_data;

    // #[test]
    // fn test() {
    //     let graph: blossom::Graph = [
    //         (0, vec![1, 2, 3]),
    //         (1, vec![0, 2]),
    //         (2, vec![0, 1]),
    //         (3, vec![0]),
    //     ]
    //     .iter()
    //     .collect();
    //     let matching = graph.maximum_matching();
    //     let matching_edges = matching.edges();
    //
    //     let mut matching_vertices = matching.vertices();
    //
    //     let mut graph_vertices = graph.vertices().to_vec();
    //
    //     assert_eq!(matching_vertices.sort(), graph_vertices.sort());
    //
    //     assert!(!matching_edges.contains(&(0, 1)) && !matching_edges.contains(&(1, 0)));
    //     assert!(matching_edges.contains(&(0, 3)) || matching_edges.contains(&(3, 0)));
    // }
    //
    // #[test]
    // fn test_2() {
    //     let mut graph = graph();
    //
    //     let blossom_graph = graph_to_blossom_graph(&graph);
    //     let matching = blossom_graph.maximum_matching();
    //     let matching_edges = matching.edges();
    //     let mut matching_vertices = matching.vertices();
    //     let mut graph_vertices = blossom_graph.vertices().to_vec();
    //     assert_eq!(matching_vertices.sort(), graph_vertices.sort());
    //
    //     for matching_edge in matching_edges {
    //         graph.remove_edge(matching_edge.0, matching_edge.1);
    //     }
    //
    //     let blossom_graph = graph_to_blossom_graph(&graph);
    //     let matching = blossom_graph.maximum_matching();
    //     let matching_edges = matching.edges();
    //     let mut matching_vertices = matching.vertices();
    //     let mut graph_vertices = blossom_graph.vertices().to_vec();
    //     assert_eq!(matching_vertices.sort(), graph_vertices.sort());
    //     for matching_edge in matching_edges {
    //         graph.remove_edge(matching_edge.0, matching_edge.1);
    //     }
    //
    //     let blossom_graph = graph_to_blossom_graph(&graph);
    //     let matching = blossom_graph.maximum_matching();
    //     let matching_edges = matching.edges();
    //     let mut matching_vertices = matching.vertices();
    //     let mut graph_vertices = blossom_graph.vertices().to_vec();
    //     assert_eq!(matching_vertices.sort(), graph_vertices.sort());
    //     for matching_edge in matching_edges {
    //         graph.remove_edge(matching_edge.0, matching_edge.1);
    //     }
    //
    //     //let edges = graph.edges();
    //     println!();
    // }
    //
    // fn graph_to_blossom_graph(graph: &SimpleGraph) -> blossom::Graph {
    //     // let graph: blossom::Graph::new();
    //     // let result_graph = blossom::graph::AnnotatedGraph::new();
    //
    //     let mut blossom_graph = Vec::with_capacity(graph.size());
    //
    //     for vertex in graph.vertices() {
    //         let mut neighbors = vec![];
    //         for edge in graph.edges_of_vertex(vertex.index()) {
    //             let neighbor = if edge.from() == vertex.index() {
    //                 edge.to()
    //             } else {
    //                 edge.from()
    //             };
    //             neighbors.push(neighbor);
    //         }
    //         blossom_graph.push((vertex.index(), neighbors));
    //     }
    //
    //     let result: blossom::Graph = blossom_graph.iter().collect();
    //     result
    // }

    // fn graph() -> SimpleGraph {
    //     let mut graph = SimpleGraph::new();
    //     graph.add_edge(0, 1);
    //     graph.add_edge(0, 4);
    //     graph.add_edge(0, 5);
    //     graph.add_edge(1, 2);
    //     graph.add_edge(1, 6);
    //     graph.add_edge(2, 3);
    //     graph.add_edge(2, 7);
    //     graph.add_edge(3, 4);
    //     graph.add_edge(3, 8);
    //     graph.add_edge(4, 9);
    //     graph.add_edge(5, 6);
    //     graph.add_edge(5, 9);
    //     graph.add_edge(6, 7);
    //     graph.add_edge(7, 8);
    //     graph.add_edge(8, 9);
    //     graph
    // }

    #[test]
    fn test_matching_colouriser() {
        let graph = test_data::get_petersen_graph();
        let is_colourable = MatchingColouriser::is_colorable(&graph);
        assert_eq!(is_colourable, false);
    }

    #[test]
    fn should_be_snark_bfs() {
        let graph = test_data::get_petersen_graph();
        let result = MatchingColouriser::is_colorable(&graph);
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_20);
        let result = MatchingColouriser::is_colorable(&graph.unwrap());
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_36);
        let result = MatchingColouriser::is_colorable(&graph.unwrap());
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_40);
        let result = MatchingColouriser::is_colorable(&graph.unwrap());
        assert_eq!(result, false);
    }

    #[test]
    fn should_be_colourable_bfs() {
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_18);
        let result = MatchingColouriser::is_colorable(&graph.unwrap());
        assert_eq!(result, true);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_112);
        let result = MatchingColouriser::is_colorable(&graph.unwrap());
        assert_eq!(result, true);
    }

    use crate::service::chromatic_properties::critical_prop::CriticalProperties;

    #[test]
    fn should_be_critical() {
        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_26_CRITICAL_1).unwrap();
        let mut props =
            CriticalProperties::of_graph_with_colourizer(&graph, MatchingColouriser::new());
        assert_eq!(props.is_critical(), true);
        assert_eq!(props.is_cocritical(), true);
        assert_eq!(props.is_vertex_subcritical(), true);
        assert_eq!(props.is_edge_subcritical(), true);

        // let graph: SimpleGraph =
        //     G6Reader::read_graph(test_data::SNARK_IN_G6_26_CRITICAL_2).unwrap();
        // let mut props =
        //     CriticalProperties::of_graph_with_colourizer(&graph, MatchingColouriser::new());
        // assert_eq!(props.is_critical(), true);
        // assert_eq!(props.is_cocritical(), true);
        // assert_eq!(props.is_vertex_subcritical(), true);
        // assert_eq!(props.is_edge_subcritical(), true);
    }

    #[test]
    fn test_3() {
        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_26_CRITICAL_1).unwrap();
        let graph = test_data::get_petersen_graph();

        let graph: SimpleGraph = G6Reader::read_graph(test_data::NO_SNARK_IN_G6_18).unwrap();

        // let matchings = find_perfect_matchings(&graph);

        let colourable = MatchingColouriser::is_colorable(&graph);

        println!("colourable: {}", colourable);
    }

    // use petgraph::Graph;
    use petgraph::visit::{Bfs, Walker};
    // use petgraph::stable_graph::StableGraph;
    use crate::graph::undirected::edge::UndirectedEdge;
    use crate::service::matching::perfect_matchings::{BfsGraph, MatchingGraph};
    use petgraph::{stable_graph, IntoWeightedEdge, Undirected};
    use serde::export::Option::Some;
    // use petgraph::stable_graph::NodeIndex;

    #[test]
    fn test_4() {
        let mut graph = petgraph::Graph::<_, ()>::new();
        let a = graph.add_node(0);

        let mut bfs = Bfs::new(&graph, a);
        while let Some(nx) = bfs.next(&graph) {
            // we can access `graph` mutably here still
            graph[nx] += 1;
        }

        // bfs.stack

        assert_eq!(graph[a], 1);

        let graph = test_data::get_petersen_graph();
        // let graph = petgraph::Graph::new();
        // let mut undirected = stable_graph::StableGraph::<u8, u16, Undirected, u8>::with_capacity(nodes, edges);
        let mut undirected =
            stable_graph::StableGraph::<u8, u16, Undirected, u8>::from_edges(graph.edges());

        println!("{:?}", undirected);

        // let start = undirected.node
        let start = stable_graph::NodeIndex::new(0);
        let mut bfs = Bfs::new(&undirected, start);
        // println!("{:?}", bfs);

        // let mut next = bfs.next(&undirected);
        // while next.is_some() {
        //     let old_next = next.unwrap();
        //     undirected.remove_node(old_next);
        //     next = bfs.next(&undirected);
        // }

        while let Some(nx) = bfs.next(&undirected) {
            // we can access `graph` mutably here still
            println!("{:?}", nx);

            // undirected.remove_node(nx);
        }

        println!("{:?}", bfs.stack);
        println!("{:?}", bfs.discovered);

        println!("{:?}", bfs.discovered.len());
    }

    impl petgraph::IntoWeightedEdge<u16> for &UndirectedEdge {
        type NodeId = u8;

        fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, u16) {
            (self.from() as u8, self.to() as u8, 0)
        }
    }

    #[test]
    fn should_traverse_using_bfs() {
        let graph = test_data::get_petersen_graph();

        let mut match_graph = MatchingGraph::new();
        for edge in graph.edges() {
            match_graph.add_edge(edge.from(), edge.to());
        }

        let mut bfs_graph = BfsGraph::new(&match_graph, 0);

        while let Some(vertex) = bfs_graph.bfs_next() {
            println!("{}", vertex);
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
        match_graph.add_vertex(0);
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, true);
        match_graph.add_vertex(1);
        let has = match_graph.has_odd_size_component();
        assert_eq!(has, true);
    }

    fn add_edges_to_graph(graph: &mut MatchingGraph, edges: Vec<UndirectedEdge>) {
        for edge in edges {
            graph.add_edge(edge.from(), edge.to());
        }
    }

    fn graph_to_matching_graph(graph: &SimpleGraph) -> MatchingGraph {
        let mut match_graph = MatchingGraph::new();
        for edge in graph.edges() {
            match_graph.add_edge(edge.from(), edge.to());
        }
        match_graph
    }

    #[test]
    fn should_find_all_perfect_matchings() {
        let graph = graph_to_matching_graph(&test_data::get_petersen_graph());

        let mut matchings = graph.perfect_matchings();

        // matchings.sort();
        // matchings.dedup();

        println!("{:?}", matchings);

        let graph = graph_to_matching_graph(&test_data::get_colorable_graph_20());
        let mut matchings = graph.perfect_matchings();
        println!("{:?}", matchings);
    }




}
