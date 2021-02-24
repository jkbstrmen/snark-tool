#[cfg(test)]
pub mod colour_tests {
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::colour::recursive::bfs_basic::BFSColouriserBasic;
    use crate::service::colour::colouriser::Colouriser;
    use crate::service::colour::cvd::cvd;
    use crate::service::colour::cvd::cvd_dfs::CvdDfsColourizer;
    use crate::service::colour::recursive::dfs_improved::DFSColourizer;
    use crate::service::colour::matchings::matching_col::MatchingColouriser;
    use crate::service::colour::sat::sat::SATColourizer;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::io::reader_s6::S6Reader;
    use crate::test::test_data::test_data;
    use crate::service::colour::recursive::bfs_improved::BFSColourizerImproved;

    #[test]
    fn should_be_snark_bfs() {
        let graph = test_data::get_petersen_graph();
        let result = DFSColourizer::is_colorable(&graph);
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_20);
        let result = DFSColourizer::is_colorable(&graph.unwrap());
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_36);
        let result = DFSColourizer::is_colorable(&graph.unwrap());
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_40);
        let result = DFSColourizer::is_colorable(&graph.unwrap());
        assert_eq!(result, false);
    }

    #[test]
    fn should_be_colourable_bfs() {
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_18);
        let result = DFSColourizer::is_colorable(&graph.unwrap());
        assert_eq!(result, true);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_112);
        let result = DFSColourizer::is_colorable(&graph.unwrap());
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
    fn should_be_colourable_cvd() {
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_18).unwrap();
        let result = cvd::is_colorable(&graph);
        assert_eq!(result, Some(true));
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_112).unwrap();
        let result = cvd::is_colorable(&graph);
        assert_eq!(result, Some(true));
        let graph = test_data::get_colorable_graph_20();
        let result = cvd::is_colorable(&graph);
        assert_eq!(result, Some(true));
    }

    #[test]
    fn should_be_unknown_cvd() {
        let graph =
            G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();
        let result = cvd::is_colorable(&graph);
        assert_eq!(result, None);
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_20).unwrap();
        let result = cvd::is_colorable(&graph);
        assert_eq!(result, None);
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_36).unwrap();
        let result = cvd::is_colorable(&graph);
        assert_eq!(result, None);
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_40).unwrap();
        let result = cvd::is_colorable(&graph);
        assert_eq!(result, None);
    }

    // new

    #[test]
    fn should_be_colourable_cvd_dfs() {
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_18).unwrap();
        let result = CvdDfsColourizer::is_colorable(&graph);
        assert_eq!(result, true);
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_112).unwrap();
        let result = CvdDfsColourizer::is_colorable(&graph);
        assert_eq!(result, true);
        let graph = test_data::get_colorable_graph_20();
        let result = CvdDfsColourizer::is_colorable(&graph);
        assert_eq!(result, true);
    }

    #[test]
    fn should_be_snark_cvd_dfs() {
        let graph =
            G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();
        let result = CvdDfsColourizer::is_colorable(&graph);
        assert_eq!(result, false);
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_20).unwrap();
        let result = CvdDfsColourizer::is_colorable(&graph);
        assert_eq!(result, false);
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_36).unwrap();
        let result = CvdDfsColourizer::is_colorable(&graph);
        assert_eq!(result, false);
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_40).unwrap();
        let result = CvdDfsColourizer::is_colorable(&graph);
        assert_eq!(result, false);
    }

    #[test]
    fn should_be_snark_matching() {
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
    fn should_be_colourable_matching() {
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_18);
        let result = MatchingColouriser::is_colorable(&graph.unwrap());
        assert_eq!(result, true);

        // let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_112);
        // let result = MatchingColouriser::is_colorable(&graph.unwrap());
        // assert_eq!(result, true);
    }

    #[test]
    fn should_be_snark_bfs_basic() {
        let graph = test_data::get_petersen_graph();
        let result = BFSColouriserBasic::is_colorable(&graph);
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_20);
        let result = BFSColouriserBasic::is_colorable(&graph.unwrap());
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_36);
        let result = BFSColouriserBasic::is_colorable(&graph.unwrap());
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_40);
        let result = BFSColouriserBasic::is_colorable(&graph.unwrap());
        assert_eq!(result, false);
    }

    #[test]
    fn should_be_colourable_bfs_basic() {
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_18);
        let result = BFSColouriserBasic::is_colorable(&graph.unwrap());
        assert_eq!(result, true);

        // let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_112);
        // let result = BFSColouriserBasic::is_colorable(&graph.unwrap());
        // // let result = DFSColourizer::is_colorable(&graph.unwrap());
        // assert_eq!(result, true);

        // let gr = ":qc@Wo]YJes?[EeD_?W?IDW@AX_}EIFSWGaMVeExxuHaOppXY]QPTsAEULIP`QEKTOPAYI?BAq@pMaVNrApySQkERcO_mgCb^";
        // let graph = S6Reader::<SimpleGraph>::read_graph(gr);
        // let result = BFSColouriserBasic::is_colorable(&graph.unwrap());
        // assert_eq!(result, true);
    }

    #[test]
    fn should_be_snark_bfs_improved() {
        let graph = test_data::get_petersen_graph();
        let result = BFSColourizerImproved::is_colorable(&graph);
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_20);
        let result = BFSColourizerImproved::is_colorable(&graph.unwrap());
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_36);
        let result = BFSColourizerImproved::is_colorable(&graph.unwrap());
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_40);
        let result = BFSColourizerImproved::is_colorable(&graph.unwrap());
        assert_eq!(result, false);
    }

    #[test]
    fn should_be_colourable_bfs_improved() {
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_18);
        let result = BFSColourizerImproved::is_colorable(&graph.unwrap());
        assert_eq!(result, true);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_112);
        let result = BFSColourizerImproved::is_colorable(&graph.unwrap());
        // let result = DFSColourizer::is_colorable(&graph.unwrap());
        assert_eq!(result, true);

        let gr = ":qc@Wo]YJes?[EeD_?W?IDW@AX_}EIFSWGaMVeExxuHaOppXY]QPTsAEULIP`QEKTOPAYI?BAq@pMaVNrApySQkERcO_mgCb^";
        let graph = S6Reader::<SimpleGraph>::read_graph(gr);
        let result = BFSColourizerImproved::is_colorable(&graph.unwrap());
        assert_eq!(result, true);
    }
}
