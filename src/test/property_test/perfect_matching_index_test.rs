#[cfg(test)]
pub mod perfect_matching_index_tests {
    use crate::graph::edge::{Edge, EdgeConstructor};
    use crate::graph::undirected::edge::UndirectedEdge;
    use crate::service::colour::colouriser::Colouriser;
    use crate::service::colour::matchings::matching_col::MatchingColouriser;
    use crate::service::colour::matchings::matching_col_2::MatchingColouriser2;
    use crate::service::colour::recursive::bfs_improved::BFSColourizerImproved;
    use crate::service::matching::perfect_matchings::{Matching, MatchingGraph};
    use crate::service::property::perfect_matching_index::{compute_matching_similarity, distinct_edges, perfect_matching_index, perfect_matching_index_1, perfect_matching_index_2};
    use crate::test::test_data::test_data;
    use serde::export::Option::Some;
    use crate::service::io::reader_g6::G6Reader;
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use std::time::Instant;

    #[test]
    fn temp() {
        let graph = test_data::get_colorable_graph_20();
        let pmi = perfect_matching_index(&graph);
        assert_eq!(pmi, 3);

        let graph = test_data::get_petersen_graph();
        let pmi = perfect_matching_index(&graph);
        assert_eq!(pmi, 7);

        let gr = test_data::SNARK_IN_G6_18;
        let graph: SimpleGraph = G6Reader::read_graph(gr).unwrap();
        let pmi = perfect_matching_index(&graph);
        assert_eq!(pmi, 4);

        let gr = test_data::SNARK_IN_G6_20;
        let graph: SimpleGraph = G6Reader::read_graph(gr).unwrap();
        let pmi = perfect_matching_index(&graph);
        assert_eq!(pmi, 4);

        let gr = test_data::SNARK_IN_G6_22;
        let graph: SimpleGraph = G6Reader::read_graph(gr).unwrap();
        let pmi = perfect_matching_index(&graph);
        assert_eq!(pmi, 4);

        let gr = test_data::SNARK_IN_G6_30;
        let graph: SimpleGraph = G6Reader::read_graph(gr).unwrap();
        let pmi = perfect_matching_index(&graph);
        assert_eq!(pmi, 4);

        // let graph = test_data::get_falcon_graph();
        // let pmi = perfect_matching_index(&graph);
        // assert_eq!(pmi, 7);

        // let colorable = BFSColourizerImproved::is_colorable(&graph);
        // let colorable = MatchingColouriser::is_colorable(&graph);
        // let colorable = MatchingColouriser2::is_colorable(&graph);
        // assert_eq!(colorable, true);
    }


    #[test]
    fn measurement() {
        let begin = Instant::now();

        let gr = test_data::SNARK_IN_G6_22;
        let graph: SimpleGraph = G6Reader::read_graph(gr).unwrap();
        // let pmi = perfect_matching_index(&graph);
        let pmi = perfect_matching_index_1(&graph);
        // let pmi = perfect_matching_index_2(&graph);
        assert_eq!(pmi, 4);

        println!("elapsed: {}", begin.elapsed().as_millis());
    }


    #[test]
    fn should_have_pmi_five() {
        let graph = test_data::get_petersen_graph();
        let pmi = perfect_matching_index(&graph);
        assert_eq!(pmi, 5);

        let graph = test_data::get_colorable_graph_20();
        // let graph = test_data::get_falcon_graph();

        let mut matching_graph = MatchingGraph::from_graph(&graph);
        let mut perfect_matchings = matching_graph.perfect_matchings();
        // println!("{}", perfect_matchings.len());

        for mut perfect_matching in perfect_matchings.iter_mut() {
            perfect_matching.edges.sort();
        }
        perfect_matchings.sort();

        for perfect_matching in perfect_matchings.iter() {
            // println!("{:?}", perfect_matching);
            for edge in perfect_matching.edges.iter() {
                print!("({}, {}) ", edge.from(), edge.to());
            }
            println!();
        }

        println!();
        let matching = perfect_matchings.pop().unwrap();
        for edge in matching.edges.iter() {
            print!("({}, {}) ", edge.from(), edge.to());
        }

        println!();
        for perfect_matching in perfect_matchings.iter() {
            let similarity = compute_matching_similarity(&matching, perfect_matching);
            println!("{}", similarity);
        }

        // let pmi = perfect_matching_index(&graph);
        // assert_eq!(pmi, 3);
    }

    #[test]
    fn should_find_distinct_edges() {
        let first = vec![
            UndirectedEdge::new(0, 1),
            UndirectedEdge::new(4, 6),
            UndirectedEdge::new(8, 15),
            UndirectedEdge::new(9, 20),
            UndirectedEdge::new(12, 13),
            UndirectedEdge::new(15, 16),
        ];
        let second = vec![
            UndirectedEdge::new(4, 6),
            UndirectedEdge::new(8, 15),
            UndirectedEdge::new(12, 13),
        ];

        let distinct = vec![
            UndirectedEdge::new(0, 1),
            UndirectedEdge::new(9, 20),
            UndirectedEdge::new(15, 16),
        ];

        let found = distinct_edges(&first, &second);
        assert_eq!(distinct, found);
    }
}
