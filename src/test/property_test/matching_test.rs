#[cfg(test)]
pub mod matching_tests {
    use crate::graph::edge::Edge;
    use crate::graph::graph::{Graph, GraphConstructor};
    use crate::graph::undirected::simple_graph::SimpleGraph;
    use crate::graph::vertex::Vertex;
    use crate::service::colour::colouriser::Colourizer;
    use crate::service::colour::matching::{find_perfect_matchings, MatchingColouriser};
    use crate::service::io::reader_g6::G6Reader;
    use crate::test::test_data::test_data;

    #[test]
    fn test() {
        let graph: blossom::Graph = [
            (0, vec![1, 2, 3]),
            (1, vec![0, 2]),
            (2, vec![0, 1]),
            (3, vec![0]),
        ]
        .iter()
        .collect();
        let matching = graph.maximum_matching();
        let matching_edges = matching.edges();

        let mut matching_vertices = matching.vertices();

        let mut graph_vertices = graph.vertices().to_vec();

        assert_eq!(matching_vertices.sort(), graph_vertices.sort());

        assert!(!matching_edges.contains(&(0, 1)) && !matching_edges.contains(&(1, 0)));
        assert!(matching_edges.contains(&(0, 3)) || matching_edges.contains(&(3, 0)));
    }

    #[test]
    fn test_2() {
        let mut graph = graph();

        let blossom_graph = graph_to_blossom_graph(&graph);
        let matching = blossom_graph.maximum_matching();
        let matching_edges = matching.edges();
        let mut matching_vertices = matching.vertices();
        let mut graph_vertices = blossom_graph.vertices().to_vec();
        assert_eq!(matching_vertices.sort(), graph_vertices.sort());

        for matching_edge in matching_edges {
            graph.remove_edge(matching_edge.0, matching_edge.1);
        }

        let blossom_graph = graph_to_blossom_graph(&graph);
        let matching = blossom_graph.maximum_matching();
        let matching_edges = matching.edges();
        let mut matching_vertices = matching.vertices();
        let mut graph_vertices = blossom_graph.vertices().to_vec();
        assert_eq!(matching_vertices.sort(), graph_vertices.sort());
        for matching_edge in matching_edges {
            graph.remove_edge(matching_edge.0, matching_edge.1);
        }

        let blossom_graph = graph_to_blossom_graph(&graph);
        let matching = blossom_graph.maximum_matching();
        let matching_edges = matching.edges();
        let mut matching_vertices = matching.vertices();
        let mut graph_vertices = blossom_graph.vertices().to_vec();
        assert_eq!(matching_vertices.sort(), graph_vertices.sort());
        for matching_edge in matching_edges {
            graph.remove_edge(matching_edge.0, matching_edge.1);
        }

        //let edges = graph.edges();
        println!();
    }

    fn graph_to_blossom_graph(graph: &SimpleGraph) -> blossom::Graph {
        // let graph: blossom::Graph::new();
        // let result_graph = blossom::graph::AnnotatedGraph::new();

        let mut blossom_graph = Vec::with_capacity(graph.size());

        for vertex in graph.vertices() {
            let mut neighbors = vec![];
            for edge in graph.edges_of_vertex(vertex.index()) {
                let neighbor = if edge.from() == vertex.index() {
                    edge.to()
                } else {
                    edge.from()
                };
                neighbors.push(neighbor);
            }
            blossom_graph.push((vertex.index(), neighbors));
        }

        let result: blossom::Graph = blossom_graph.iter().collect();
        result
    }

    fn graph() -> SimpleGraph {
        let mut graph = SimpleGraph::new();
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(0, 5);
        graph.add_edge(1, 2);
        graph.add_edge(1, 6);
        graph.add_edge(2, 3);
        graph.add_edge(2, 7);
        graph.add_edge(3, 4);
        graph.add_edge(3, 8);
        graph.add_edge(4, 9);
        graph.add_edge(5, 6);
        graph.add_edge(5, 9);
        graph.add_edge(6, 7);
        graph.add_edge(7, 8);
        graph.add_edge(8, 9);
        graph
    }

    #[test]
    fn test_matching_colouriser() {
        let graph = test_data::get_petersen_graph();

        let is_colourable = MatchingColouriser::is_colorable(&graph);

        println!("{}", is_colourable);
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

        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_26_CRITICAL_2).unwrap();
        let mut props =
            CriticalProperties::of_graph_with_colourizer(&graph, MatchingColouriser::new());
        assert_eq!(props.is_critical(), true);
        assert_eq!(props.is_cocritical(), true);
        assert_eq!(props.is_vertex_subcritical(), true);
        assert_eq!(props.is_edge_subcritical(), true);
    }

    #[test]
    fn test_3() {
        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_26_CRITICAL_1).unwrap();
        let graph = test_data::get_petersen_graph();

        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::NO_SNARK_IN_G6_18).unwrap();

        // let matchings = find_perfect_matchings(&graph);

        let colourable = MatchingColouriser::is_colorable(&graph);

        println!("colourable: {}", colourable);

    }

    // use petgraph::Graph;
    use petgraph::visit::Bfs;
    // use petgraph::stable_graph::StableGraph;
    use petgraph::{stable_graph, Undirected, IntoWeightedEdge};
    use crate::graph::undirected::edge::UndirectedEdge;
    // use petgraph::stable_graph::NodeIndex;

    #[test]
    fn test_4() {

        let mut graph = petgraph::Graph::<_,()>::new();
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
        let mut undirected = stable_graph::StableGraph::<u8, u16, Undirected, u8>::from_edges(graph.edges());

        println!("{:?}", undirected);

        // let start = undirected.node
        let start = stable_graph::NodeIndex::new(0);
        let mut bfs = Bfs::new(&undirected, start);
        // println!("{:?}", bfs);
        // while let Some(nx) = bfs.next(&undirected) {
        //     // we can access `graph` mutably here still
        //     println!("{:?}", nx);
        // }

        println!("{:?}", bfs.stack);
        println!("{:?}", bfs.discovered.len());

    }

    impl petgraph::IntoWeightedEdge<u16> for &UndirectedEdge {
        type NodeId = u8;

        fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, u16) {
            (self.from() as u8, self.to() as u8, 0)
        }
    }
}
