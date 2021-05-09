#[cfg(test)]
pub mod order_tests {
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::io::reader_g6::G6Reader;
    use crate::service::property::order::is_cubic;
    use crate::tests::test_data::test_data;

    #[test]
    fn should_be_cubic() {
        let graph = test_data::get_petersen_graph();
        let cubic = is_cubic(graph);
        assert_eq!(cubic, true);

        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_36_STABLE_RES_3).unwrap();
        let cubic = is_cubic(graph);
        assert_eq!(cubic, true);
    }
}
