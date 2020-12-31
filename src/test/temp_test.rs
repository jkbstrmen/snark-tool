#[cfg(test)]
mod matchings_performance {
    use crate::graph::edge::Edge;
    use crate::graph::graph::{Graph, GraphConstructor};
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::chromatic_properties::critical_prop::CriticalProperties;
    use crate::service::chromatic_properties::resistance::Resistance;
    use crate::service::chromatic_properties::resistibility::Resistibility;
    use crate::service::colour::colouriser::Colouriser;
    use crate::service::colour::matching_col::MatchingColouriser;
    use crate::service::io::reader::Reader;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::io::writer_g6::G6Writer;
    use crate::service::matching::perfect_matchings::{Matching, MatchingGraph};
    use crate::service::property::oddness::Oddness;
    use crate::test::test_data::test_data;
    use std::collections::HashMap;
    use std::time::Instant;
    use std::{fs, time};
    use crate::service::chromatic_properties::stable_and_critical_prop::StableAndCriticalProperties;
    use crate::service::colour::sat::SATColourizer;
    use crate::service::colour::dfs_improved::DFSColourizer;
    use crate::graph::undirected::simple_graph::graph::SimpleSparseGraph;
    use crate::service::property::cyclic_connectivity::cyclic_edge_connectivity;

    #[test]
    fn perfect_matchings_performance() {
        let path = "resources/graphs/Generated_100_36vert_snarks.g6";
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        let begin = Instant::now();

        while let Some(graph_result) = reader.next() {
            let graph = graph_result.unwrap();

            // println!("Here");
            // let colourable = BFSColourizer::is_colorable(&graph);
            // let colourable = MatchingColouriser::is_colorable(&graph);

            let mut match_graph = MatchingGraph::from_graph(&graph);
            let mut matchings = match_graph.perfect_matchings();

            // let res_tester = Resistance::new_with_colouriser(BFSColourizer::new());
            // let resistance = res_tester.vertex_resistance(&graph);
        }

        println!("elapsed: {}ms", begin.elapsed().as_millis());
    }

    #[test]
    fn matching_colouriser_performance() {
        let path = "resources/graphs/Generated_500_36vert_snarks.g6";
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        let begin = Instant::now();

        while let Some(graph_result) = reader.next() {
            let graph = graph_result.unwrap();
            // let colourable = BFSColourizer::is_colorable(&graph);
            let colourable = MatchingColouriser::is_colorable(&graph);
            if colourable {
                println!("error");
            }
        }
        println!("elapsed: {}ms", begin.elapsed().as_millis());
    }

    #[test]
    fn colouriser_performance() {
        // let path = "resources/graphs/Generated_2100_36vert_snarks.g6";
        let path = "resources/graphs/Generated_10000_36vert_snarks_04.g6";
        // let path = "resources/graphs/Generated_10_36vert_snarks.g6";
        // let path = "resources/graphs/Generated_10000_34vert_snarks_05.g6";
        // let path = "resources/graphs/Generated_100000_34vert_snarks_05.g6";
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        let begin = Instant::now();

        let mut all_false = true;
        while let Some(graph_result) = reader.next() {
            let mut graph = graph_result.unwrap();
            // graph.remove_edges_of_vertex(0);
            // graph.remove_edges_of_vertex(1);

            let colourable = DFSColourizer::is_colorable(&graph);
            // let colourable = BFSColourizer2::is_colorable(&graph);
            // let colourable = MatchingColouriser::is_colorable(&graph);
            if colourable {
                all_false = false;
            }
        }
        println!("all false: {}", all_false);
        println!("elapsed: {}ms", begin.elapsed().as_millis());
    }

    // use crate::service::colour::matching::ELAPSED;
    // use crate::service::colour::matching::ELAPSED_2;
    // use crate::service::colour::matching::HAS_COUNTER;
    #[test]
    fn criticality_performance() {
        // let path = "resources/graphs/Generated_10_36vert_snarks.g6";
        // let path = "resources/graphs/Generated_100_36vert_snarks.g6";
        let path = "resources/graphs/100K.Generated_graphs.30.04.sn.cyc4.g6.36.04-STABLE.g6";
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        let begin = Instant::now();

        while let Some(graph_result) = reader.next() {
            let graph = graph_result.unwrap();

            // let colourable = MatchingColouriser::is_colorable(&graph);

            let mut props =
                // CriticalProperties::of_graph_with_colourizer(&graph, BFSColourizer::new());
                // CriticalProperties::of_graph_with_colourizer(&graph, BFSColourizer2::new());
                StableAndCriticalProperties::of_graph_with_colourizer(&graph, DFSColourizer::new());
                // StableAndCriticalProperties::of_graph_with_colourizer(&graph, BFSColourizer2::new());
            // CriticalProperties::of_graph_with_colourizer(&graph, MatchingColouriser::new());
            let crit = props.is_critical();
            let cocrit = props.is_cocritical();
            let e_subcrit = props.is_edge_subcritical();

            let stable = props.is_stable();

            if crit || cocrit {
                println!("{}, {}", crit, cocrit);
            }
        }

        println!("elapsed: {}ms", begin.elapsed().as_millis());
        // unsafe { println!("elapsed on colouring: {}us", ELAPSED); }

        // unsafe {
        //     println!("elapsed 1: {}us", ELAPSED);
        //     println!("elapsed 2: {}ns", ELAPSED_2);
        //     println!("elapsed 3: {}ns", ELAPSED_3);
        //     // println!("has counter: {}", HAS_COUNTER);
        //     // println!("has not counter: {}", HAS_NOT_COUNTER);
        // }
    }

    #[test]
    fn oddness_performance() {
        let path = "resources/graphs/Generated_100_36vert_snarks.g6";
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        let begin = Instant::now();

        while let Some(graph_result) = reader.next() {
            let graph = graph_result.unwrap();

            let oddness = Oddness::of_graph(&graph);
        }

        println!("elapsed: {}ms", begin.elapsed().as_millis());
    }

    #[test]
    fn resistance_performance() {
        let path = "resources/graphs/Generated_100_36vert_snarks.g6";
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        let begin = Instant::now();

        while let Some(graph_result) = reader.next() {
            let graph = graph_result.unwrap();

            // let res_tester = Resistance::new_with_colouriser(BFSColourizer::new());
            let res_tester = Resistance::new_with_colouriser(MatchingColouriser::new());
            let resistance = res_tester.vertex_resistance(&graph);

            assert_eq!(resistance.unwrap(), 2);
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

        let mut match_graph = MatchingGraph::from_graph(&graph);
        let perf_matchings = match_graph.perfect_matchings();

        let mut edges_occurence = HashMap::new();
        for perf_matching in perf_matchings {
            for edge in perf_matching.edges {
                if let Some(edge_occ) = edges_occurence.get_mut(&edge) {
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

        for key in map.keys() {}
    }

    #[test]
    fn bfs2_test() {

        let begin = Instant::now();

        // let colourizer = BFSColourizer2::new();
        let colourizer = DFSColourizer::new();
        let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_34_STABLE_1).unwrap();
        let mut props = CriticalProperties::of_graph_with_colourizer(&graph, colourizer);
        assert_eq!(props.is_critical(), false);
        assert_eq!(props.is_cocritical(), false);
        assert_eq!(props.is_vertex_subcritical(), false);
        assert_eq!(props.is_edge_subcritical(), false);

        println!("elapsed: {}ms", begin.elapsed().as_millis());

    }


    #[test]
    fn tuning_bfs() {

        println!("hello");

        // let path = "resources/graphs/Generated_2100_36vert_snarks.g6";
        // let path = "resources/graphs/Generated_10000_36vert_snarks_04.g6";
        // let path = "resources/graphs/Generated_10000_34vert_snarks_05.g6";
        let path = "resources/graphs/snark.36.cyc3.res3.g6";
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        let begin = Instant::now();

        let mut all_false = true;
        while let Some(graph_result) = reader.next() {
            let mut graph = graph_result.unwrap();
            graph.remove_edges_of_vertex(2);
            graph.remove_edges_of_vertex(15);

            for i in 0..1 {

                let colourable = DFSColourizer::is_colorable(&graph);
                // let colourable = MatchingColouriser::is_colorable(&graph);
                // let colourable = SATColourizer::is_colorable(&graph);
                if colourable {
                    all_false = false;
                }

            }
        }
        println!("all false: {}", all_false);
        println!("elapsed: {}ms", begin.elapsed().as_millis());

        // unsafe { println!("counter: {}", COUNTER); }
    }

    #[test]
    fn temp() {

        let graph = G6Reader::<SimpleGraph>::read_graph("ks???OC@_P?B?g?O??GG??_?@??O???@???P?@_??@G???G??C????_??@????G????_G??G@??A??_???K????_@???A?_????W???A_?????c?????W?C????CQ?????oC?????OO????CA??????H??????S").unwrap();

        println!("{}", graph.size());


    }
    #[test]
    fn criticality_performance_2() {
        // let path = "resources/graphs/Generated_10_36vert_snarks.g6";
        let path = "resources/graphs/Generated_100_36vert_snarks.g6";
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        let begin = Instant::now();

        let mut crit_count = 0;
        let mut cocrit_count = 0;
        let mut v_sub_count = 0;
        let mut e_sub_count = 0;
        let mut stab_count = 0;
        let mut costab_count = 0;
        while let Some(graph_result) = reader.next() {
            let graph = graph_result.unwrap();

            // let colourable = MatchingColouriser::is_colorable(&graph);

            let mut props =
                // CriticalProperties::of_graph_with_colourizer(&graph, BFSColourizer::new());
                // CriticalProperties::of_graph_with_colourizer(&graph, BFSColourizer2::new());
                StableAndCriticalProperties::of_graph_with_colourizer(&graph, DFSColourizer::new());
            // StableAndCriticalProperties::of_graph_with_colourizer(&graph, BFSColourizer2::new());
            // CriticalProperties::of_graph_with_colourizer(&graph, MatchingColouriser::new());
            let crit = props.is_critical();
            let cocrit = props.is_cocritical();
            let e_subcrit = props.is_edge_subcritical();
            let v_subcrit = props.is_vertex_subcritical();
            let stable = props.is_stable();
            let costable = props.is_costable();

            if crit {crit_count += 1;}
            if cocrit {cocrit_count += 1;}
            if v_subcrit {v_sub_count += 1;}
            if e_subcrit {e_sub_count += 1;}
            if stable {stab_count += 1;}
            if costable {costab_count += 1;}
        }

        println!("CRITICAL: {}", crit_count);
        println!("COCRITICAL: {}", cocrit_count);
        println!("V subCRITICAL: {}", v_sub_count);
        println!("E subCRITICAL: {}", e_sub_count);
        println!("STABLE: {}", stab_count);
        println!("COSTABLE: {}", costab_count);

        println!("elapsed: {}ms", begin.elapsed().as_millis());
        // unsafe { println!("elapsed on colouring: {}us", ELAPSED); }

        // unsafe {
        //     println!("elapsed 1: {}us", ELAPSED);
        //     println!("elapsed 2: {}ns", ELAPSED_2);
        //     println!("elapsed 3: {}ns", ELAPSED_3);
        //     // println!("has counter: {}", HAS_COUNTER);
        //     // println!("has not counter: {}", HAS_NOT_COUNTER);
        // }
    }


    #[test]
    fn cyclic_edge_connectivity_performance() {

        let path = test_data::GG_30_G05_CYC5_G6_100_FILE_PATH;
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleSparseGraph>::new(&file_result);

        let begin = Instant::now();

        let mut counter = 0;
        while let Some(graph_result) = reader.next() {
            let graph = graph_result.unwrap();

            let colourable = DFSColourizer::is_colorable(&graph);
            assert_eq!(colourable, false);

            // let cec = cyclic_edge_connectivity(&graph);
            // assert_eq!(cec.is_some(), true);
            // assert_eq!(cec.unwrap(), 5);

            counter += 1;
        }

        println!("elapsed: {}", begin.elapsed().as_millis());

    }

}
