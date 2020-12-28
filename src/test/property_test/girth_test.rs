#[cfg(test)]
pub mod girth_tests {
    use crate::graph::undirected::simple_graph::SimpleGraph;
    use crate::graph::undirected_sparse::graph::SimpleSparseGraph;
    use crate::service::colour::colouriser::Colouriser;
    use crate::service::colour::dfs_improved::DFSColourizer;
    use crate::service::io::reader::Reader;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::property::girth::girth;
    use crate::test::test_data::test_data;
    use std::fs;
    use std::time::Instant;

    #[test]
    fn should_have_girth_five() {
        let graph = test_data::get_petersen_graph();
        let girth = girth(&graph);
        assert_eq!(girth, 5);
    }

    #[test]
    fn should_have_girth_six() {
        let graph_string = test_data::SNARK_IN_G6_30_GIRTH_6;
        let graph = G6Reader::<SimpleGraph>::read_graph(graph_string).unwrap();
        let girth = girth(&graph);
        assert_eq!(girth, 6);
    }

    #[test]
    fn should_have_girth_three() {
        // let path = "resources/graphs/Generated_100_36vert_snarks.g6";
        let path = "resources/graphs/Generated_graphs.30.05.sn.cyc4.g6";
        let file_result = fs::OpenOptions::new().read(true).open(&path).unwrap();
        let mut reader = G6Reader::<SimpleGraph>::new(&file_result);

        let begin = Instant::now();

        let mut counter = 0;
        let mut girth_6_counter = 0;

        while let Some(graph_result) = reader.next() {
            let graph = graph_result.unwrap();

            // let girth = girth(&graph);
            //
            // if girth == 6 {
            //     girth_6_counter += 1;
            // } else {
            //     assert_eq!(girth, 5);
            // }

            let colourable = DFSColourizer::is_colorable(&graph);
            assert_eq!(colourable, false);
        }

        // assert_eq!(girth_6_counter, 1);

        println!("elapsed: {}ms", begin.elapsed().as_millis());
    }
}
