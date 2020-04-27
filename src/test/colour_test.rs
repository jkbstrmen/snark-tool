#[cfg(test)]
pub mod colour_tests {
    use crate::graph::undirected::simple_graph::SimpleGraph;
    use crate::service::colour::bfs::BFSColourizer;
    use crate::service::colour::colouriser::Colourizer;
    use crate::service::colour::sat;
    use crate::service::io::reader::Reader;
    use crate::service::io::reader_ba::BaReader;
    use crate::service::io::reader_g6::G6Reader;
    use crate::test::test_data::test_data;
    use std::fs::OpenOptions;
    use crate::service::colour::sat::SATColourizer;

    #[test]
    fn should_be_snark_bfs() {
        let graph = test_data::get_petersen_graph();
        let result = BFSColourizer::is_colorable(&graph);
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_20);
        let result = BFSColourizer::is_colorable(&graph.unwrap());
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_36);
        let result = BFSColourizer::is_colorable(&graph.unwrap());
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_40);
        let result = BFSColourizer::is_colorable(&graph.unwrap());
        assert_eq!(result, false);
    }

    #[test]
    fn should_be_colourable_bfs() {
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_18);
        let result = BFSColourizer::is_colorable(&graph.unwrap());
        assert_eq!(result, true);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_112);
        let result = BFSColourizer::is_colorable(&graph.unwrap());
        assert_eq!(result, true);
    }

    #[test]
    fn should_be_snark_sat() {
        let graph = test_data::get_petersen_graph();
        let result = SATColourizer::is_colorable(&graph);
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_20);
        let result = SATColourizer::is_colorable(&graph.unwrap());
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_36);
        let result = SATColourizer::is_colorable(&graph.unwrap());
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_40);
        let result = SATColourizer::is_colorable(&graph.unwrap());
        assert_eq!(result, false);
    }

    #[test]
    fn should_be_colourable_sat() {
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_18);
        let result = SATColourizer::is_colorable(&graph.unwrap());
        assert_eq!(result, true);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_112);
        let result = SATColourizer::is_colorable(&graph.unwrap());
        assert_eq!(result, true);
    }

    #[test]
    fn run_cvd(){




    }
}
