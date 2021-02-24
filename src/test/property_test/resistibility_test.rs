#[cfg(test)]
pub mod resistibility_tests {
    use crate::graph::undirected::simple_graph::graph::SimpleGraph;
    use crate::service::chromatic_properties::resistibility::Resistibility;
    use crate::service::colour::colouriser::Colouriser;
    use crate::service::colour::recursive::dfs_improved::DFSColourizer;
    use crate::service::io::reader_g6::G6Reader;
    use crate::test::test_data::test_data;

    #[test]
    fn should_test_resistibility_indices() {
        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_36_STABLE_34_IER).unwrap();
        let colouriser = DFSColourizer::new();
        let mut resistibility_tester = Resistibility::of_graph_with_colouriser(&graph, colouriser);

        let vertex_resistibility_index = resistibility_tester.vertex_resistibility_index();
        let edge_resistibility_index = resistibility_tester.edge_resistibility_index();
        assert_eq!(vertex_resistibility_index, 20);
        assert_eq!(edge_resistibility_index, 34);
    }

    #[test]
    fn should_test_resistibility_indices_petersen() {
        let graph = test_data::get_petersen_graph();
        let colouriser = DFSColourizer::new();
        let mut resistibility_tester = Resistibility::of_graph_with_colouriser(&graph, colouriser);

        let vertex_resistibility_index = resistibility_tester.vertex_resistibility_index();
        let edge_resistibility_index = resistibility_tester.edge_resistibility_index();
        assert_eq!(vertex_resistibility_index, 0);
        assert_eq!(edge_resistibility_index, 0);
    }
}
