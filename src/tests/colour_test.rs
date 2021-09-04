#[cfg(test)]
pub mod colour_tests {
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::colour::colouriser::Colouriser;
    use crate::service::colour::cvd::cvd;
    use crate::service::colour::cvd::cvd_dfs::CvdDfsColourizer;
    use crate::service::colour::matchings::matching_col::MatchingColouriser;
    use crate::service::colour::matchings::matching_col_2::MatchingColouriser2;
    use crate::service::colour::recursive::bfs_basic::BFSColouriserBasic;
    use crate::service::colour::recursive::bfs_improved::BFSColourizerImproved;
    use crate::service::colour::recursive::dfs_improved::DFSColourizer;
    use crate::service::colour::sat::sat::SATColourizer;
    use crate::service::colour::sat::sat_cadical::SATColourizerCadical;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::io::reader_s6::S6Reader;
    use crate::tests::test_data::test_data;

    #[test]
    fn should_be_snark_dfs() {
        should_be_snark::<DFSColourizer>();
    }

    #[test]
    fn should_be_colourable_bfs() {
        should_be_colourable::<DFSColourizer>();
    }

    #[test]
    fn should_be_snark_sat() {
        should_be_snark::<SATColourizer>();
    }

    #[test]
    fn should_be_colourable_sat() {
        should_be_colourable::<SATColourizer>();
    }

    #[test]
    fn should_be_snark_sat_cadical() {
        should_be_snark::<SATColourizerCadical>();
    }

    #[test]
    fn should_be_colourable_sat_cadical() {
        should_be_colourable::<SATColourizerCadical>();
    }

    #[test]
    fn should_be_colourable_cvd() {
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_18).unwrap();
        let _result = cvd::is_colorable(&graph);
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_112).unwrap();
        let _result = cvd::is_colorable(&graph);
        let graph = test_data::get_colorable_graph_20();
        let _result = cvd::is_colorable(&graph);
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

    #[test]
    fn should_be_snark_cvd_dfs() {
        should_be_snark::<CvdDfsColourizer>();
    }

    #[test]
    fn should_be_colourable_cvd_dfs() {
        should_be_colourable::<CvdDfsColourizer>();
    }

    #[test]
    fn should_be_snark_matching() {
        should_be_snark::<MatchingColouriser>();
    }

    #[test]
    fn should_be_colourable_matching() {
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_18);
        let result = MatchingColouriser::is_colorable(&graph.unwrap());
        assert_eq!(result, true);

        let graph = S6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_S6_50);
        let result = MatchingColouriser::is_colorable(&graph.unwrap());
        assert_eq!(result, true);
    }

    #[test]
    fn should_be_snark_matching_2() {
        should_be_snark::<MatchingColouriser2>();
    }

    #[test]
    fn should_be_colourable_matching_2() {
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_18);
        let result = MatchingColouriser2::is_colorable(&graph.unwrap());
        assert_eq!(result, true);

        let graph = S6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_S6_50);
        let result = MatchingColouriser2::is_colorable(&graph.unwrap());
        assert_eq!(result, true);
    }

    #[test]
    fn should_be_snark_bfs_basic() {
        should_be_snark::<BFSColouriserBasic>();
    }

    #[test]
    fn should_be_colourable_bfs_basic() {
        // should_be_colourable::<BFSColouriserBasic>();
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_18);
        let result = BFSColouriserBasic::is_colorable(&graph.unwrap());
        assert_eq!(result, true);
    }

    #[test]
    fn should_be_snark_bfs_improved() {
        should_be_snark::<BFSColourizerImproved>();
    }

    #[test]
    fn should_be_colourable_bfs_improved() {
        should_be_colourable::<BFSColourizerImproved>();
    }

    fn should_be_snark<C: Colouriser>() {
        let graph = test_data::get_petersen_graph();
        let result = C::is_colorable(&graph);
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_20);
        let result = C::is_colorable(&graph.unwrap());
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_36);
        let result = C::is_colorable(&graph.unwrap());
        assert_eq!(result, false);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_40);
        let result = C::is_colorable(&graph.unwrap());
        assert_eq!(result, false);
    }

    fn should_be_colourable<C: Colouriser>() {
        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_18);
        let result = C::is_colorable(&graph.unwrap());
        assert_eq!(result, true);

        let graph = S6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_S6_50);
        let result = C::is_colorable(&graph.unwrap());
        assert_eq!(result, true);

        let graph = G6Reader::<SimpleGraph>::read_graph(test_data::NO_SNARK_IN_G6_112);
        let result = C::is_colorable(&graph.unwrap());
        assert_eq!(result, true);
    }
}
