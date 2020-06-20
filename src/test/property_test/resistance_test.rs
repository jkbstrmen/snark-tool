#[cfg(test)]
pub mod resistance_tests {
    use crate::graph::undirected::simple_graph::SimpleGraph;
    use crate::service::chromatic_properties::resistance::Resistance;
    use crate::service::colour::bfs::BFSColourizer;
    use crate::service::colour::colouriser::Colourizer;
    use crate::service::io::reader_g6::G6Reader;
    use crate::test::test_data::test_data;

    #[test]
    fn should_have_resistance_zero() {
        let res_tester = Resistance::new_with_colourizer(BFSColourizer::new());
        let graph: SimpleGraph = G6Reader::read_graph(test_data::NO_SNARK_IN_G6_18).unwrap();
        let e_resistance = res_tester.edge_resistance(&graph);
        let v_resistance = res_tester.vertex_resistance(&graph);
        assert_eq!(e_resistance.is_some(), true);
        assert_eq!(e_resistance.unwrap(), 0);
        assert_eq!(v_resistance.is_some(), true);
        assert_eq!(v_resistance.unwrap(), 0);
    }

    #[test]
    fn should_have_resistance_two() {
        let res_tester = Resistance::new_with_colourizer(BFSColourizer::new());
        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_26_CRITICAL_1).unwrap();
        let e_resistance = res_tester.edge_resistance(&graph);
        let v_resistance = res_tester.vertex_resistance(&graph);
        assert_eq!(e_resistance.is_some(), true);
        assert_eq!(e_resistance.unwrap(), 2);
        assert_eq!(v_resistance.is_some(), true);
        assert_eq!(v_resistance.unwrap(), 2);
    }

    // too long test
    // #[test]
    // fn should_have_resistance_three() {
    //     let res_tester = Resistance::new_with_colourizer(SATColourizer::new());
    //     let graph: SimpleGraph =
    //         G6Reader::read_graph(test_data::SNARK_IN_G6_76).unwrap();
    //     let v_resistance = res_tester.vertex_resistance(&graph);
    //     assert_eq!(v_resistance.is_some(), true);
    //     assert_eq!(v_resistance.unwrap(), 3);
    // }
}
