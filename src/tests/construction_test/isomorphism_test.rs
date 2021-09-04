#[cfg(test)]
pub mod isomorphism_tests {
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::constructions::isomorphism::is_isomorphic;
    use crate::service::io::reader_g6::G6Reader;
    use crate::tests::test_data::test_data;

    #[test]
    fn isomorphism_test() {
        let petersen = test_data::get_petersen_graph();
        let other = test_data::get_colorable_graph_20();
        let isomorphic = is_isomorphic(&petersen, &other);
        assert_eq!(isomorphic, false);

        let first: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_38_1).unwrap();
        let second: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_38_2).unwrap();
        let isomorphic = is_isomorphic(&first, &second);
        assert_eq!(isomorphic, false);
    }
}
