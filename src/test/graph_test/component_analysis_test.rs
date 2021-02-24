#[cfg(test)]
mod graph_component_analysis_tests {
    use crate::graph::edge::Edge;
    use crate::graph::graph::Graph;
    use crate::service::colour::colouriser::Colouriser;
    use crate::service::colour::recursive::dfs_improved::DFSColourizer;
    use crate::service::component_analysis::edge_pairs::{
        PairsOfNonAdjacentEdges, RemovablePairsOfEdges,
    };
    use crate::service::component_analysis::edge_triplets::RemovableTripletsOfEdges;
    use crate::service::component_analysis::vertex_pairs::PairsOfAdjacentVertices;
    use crate::test::test_data::test_data;

    #[test]
    fn should_produce_pairs_of_non_adjacent_edges() {
        let graph = test_data::get_petersen_graph();
        let mut non_adjacent_edges = PairsOfNonAdjacentEdges::new(&graph);
        let mut counter = 0;
        while let Some(pair) = non_adjacent_edges.next() {
            counter += 1;
            assert_eq!(pair.0.is_adjacent(pair.1), false);
        }
        assert_eq!(counter, 75);
    }

    #[test]
    fn should_produce_pairs_of_adjacent_vertices() {
        let graph = test_data::get_petersen_graph();
        let mut adjacent_vertices = PairsOfAdjacentVertices::new(&graph);
        let mut counter = 0;
        while let Some(pair) = adjacent_vertices.next() {
            println!("{:?}", pair);
            counter += 1;
        }
        assert_eq!(counter, 15);
    }

    #[test]
    fn should_produce_pairs_of_removable_edges() {
        let graph = test_data::get_petersen_graph();
        let mut local_graph = graph.clone();

        let colouriser = DFSColourizer::new();
        let pairs = RemovablePairsOfEdges::new(&graph, &colouriser);
        for pair in pairs {
            local_graph.remove_edge(pair.0.from(), pair.0.to());
            local_graph.remove_edge(pair.1.from(), pair.1.to());

            let colourable = DFSColourizer::is_colorable(&local_graph);
            assert_eq!(colourable, false);

            local_graph.add_edge(pair.0.from(), pair.0.to());
            local_graph.add_edge(pair.1.from(), pair.1.to());
        }
    }

    #[test]
    fn should_produce_triplets_of_removable_edges() {
        let graph = test_data::get_petersen_graph();
        let mut local_graph = graph.clone();

        let colouriser = DFSColourizer::new();
        let triplets = RemovableTripletsOfEdges::new(&graph, &colouriser);
        let mut triplets_vec = vec![];
        for triplet in triplets {
            local_graph.remove_edge(triplet.0.from(), triplet.0.to());
            local_graph.remove_edge(triplet.1.from(), triplet.1.to());
            local_graph.remove_edge(triplet.2.from(), triplet.2.to());

            let colourable = DFSColourizer::is_colorable(&local_graph);
            assert_eq!(colourable, false);

            local_graph.add_edge(triplet.0.from(), triplet.0.to());
            local_graph.add_edge(triplet.1.from(), triplet.1.to());
            local_graph.add_edge(triplet.2.from(), triplet.2.to());

            triplets_vec.push(triplet);
        }
        assert_eq!(triplets_vec.len(), 10);
    }
}
