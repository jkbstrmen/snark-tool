#[cfg(test)]
mod matchings_performance {
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::io::reader::Reader;
    use std::fs;
    use crate::graph::undirected::simple_graph::SimpleGraph;
    use std::time::Instant;
    use crate::service::colour::bfs::BFSColourizer;
    use crate::service::colour::colouriser::Colourizer;
    use crate::service::matching::perfect_matchings::{Matching, MatchingGraph};
    use crate::service::chromatic_properties::resistance::Resistance;
    use crate::graph::graph::{GraphConstructor, Graph};
    use crate::service::chromatic_properties::resistibility::Resistibility;
    use crate::service::io::writer_g6::G6Writer;
    use std::collections::HashMap;
    use crate::graph::edge::Edge;

    #[test]
    fn test() {

        let path = "resources/graphs/Generated_500_36vert_snarks.g6";
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();

        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        // let next = reader.next();

        let begin = Instant::now();

        while let Some(graph_result) = reader.next(){
            let graph = graph_result.unwrap();

            // println!("Here");
            // let colourable = BFSColourizer::is_colorable(&graph);

            let match_graph = MatchingGraph::from_graph(&graph);
            let mut matchings = match_graph.perfect_matchings();

            // let res_tester = Resistance::new_with_colouriser(BFSColourizer::new());
            // let resistance = res_tester.vertex_resistance(&graph);

        }

        println!("elapsed: {}ms", begin.elapsed().as_millis());

    }

    #[test]
    fn test_2() {
        let mut graph = SimpleGraph::new();
        graph.add_edge(0, 1);
        graph.add_edge(0, 3);
        graph.add_edge(0, 5);
        graph.add_edge(1, 2);
        graph.add_edge(1, 6);
        graph.add_edge(2, 4);
        graph.add_edge(2, 8);
        graph.add_edge(3, 4);
        graph.add_edge(3, 9);
        graph.add_edge(4, 7);
        graph.add_edge(5, 7);
        graph.add_edge(5, 12);
        graph.add_edge(6, 7);
        graph.add_edge(6, 22);
        graph.add_edge(8, 10);
        graph.add_edge(8, 16);
        graph.add_edge(9, 10);
        graph.add_edge(9, 23);
        graph.add_edge(10, 11);
        graph.add_edge(11, 14);
        graph.add_edge(11, 20);
        graph.add_edge(12, 13);
        graph.add_edge(12, 17);
        graph.add_edge(13, 14);
        graph.add_edge(13, 15);
        graph.add_edge(14, 18);
        graph.add_edge(15, 16);
        graph.add_edge(15, 19);
        graph.add_edge(16, 18);
        graph.add_edge(17, 18);
        graph.add_edge(17, 19);
        graph.add_edge(19, 26);
        graph.add_edge(20, 21);
        graph.add_edge(20, 25);
        graph.add_edge(21, 22);
        graph.add_edge(21, 24);
        graph.add_edge(22, 27);
        graph.add_edge(23, 24);
        graph.add_edge(23, 25);
        graph.add_edge(24, 26);
        graph.add_edge(25, 27);
        graph.add_edge(26, 27);

        let graph_string = G6Writer::graph_to_g6_string(&graph);
        println!("{}", graph_string);

        let match_graph = MatchingGraph::from_graph(&graph);
        let perf_matchings = match_graph.perfect_matchings();

        let mut edges_occurence = HashMap::new();
        for perf_matching in perf_matchings {
            for edge in perf_matching.edges {
                if let Some(edge_occ) = edges_occurence.get_mut(&edge){
                    *edge_occ += 1;
                } else {
                    edges_occurence.insert(edge, 1);
                }
            }
        }

        for edge in edges_occurence {
            println!("({} - {}): {}", (edge.0).from(), (edge.0).to(), edge.1);
        }

        println!();


        // let mut res = Resistibility::of_graph_with_colouriser(&graph, BFSColourizer::new());
        // let vert_res = res.vertices_resistibility();
        // let edge_res = res.edges_resistibility();
        //
        // for i in 0..vert_res.len() {
        //     println!("{}: {}", i, vert_res[i]);
        // }
        //
        // println!();
        //
        // for edge_re in edge_res {
        //     println!("({} - {}): {}", (edge_re.0).0, (edge_re.0).1, edge_re.1);
        // }
    }

    // let keys = self.vertices.keys();

    #[test]
    fn test_3() {

        let mut map = HashMap::new();
        map.insert(0, true);
        map.insert(1, true);
        map.insert(2, true);

        for key in map.keys() {

        }

    }
}