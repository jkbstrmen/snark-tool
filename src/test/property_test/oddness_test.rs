#[cfg(test)]
pub mod oddness_tests {
    use crate::graph::undirected::simple_graph::SimpleGraph;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::property::oddness::Oddness;
    use crate::test::test_data::test_data;

    #[test]
    fn should_have_oddness_zero() {
        let graph: SimpleGraph = G6Reader::read_graph(test_data::NO_SNARK_IN_G6_18).unwrap();
        let oddness = Oddness::of_graph(&graph);
        assert_eq!(oddness, 0);
    }

    #[test]
    fn should_have_oddness_two() {
        let graph = test_data::get_petersen_graph();
        let oddness = Oddness::of_graph(&graph);
        assert_eq!(oddness, 2);

        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_26_CRITICAL_1).unwrap();
        let oddness = Oddness::of_graph(&graph);
        assert_eq!(oddness, 2);
    }

    #[test]
    fn should_have_oddness_four() {
        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_36_STABLE_RES_3).unwrap();
        let oddness = Oddness::of_graph(&graph);
        assert_eq!(oddness, 4);
    }

    //
}
