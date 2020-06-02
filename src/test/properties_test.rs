#[cfg(test)]
pub mod properties_tests {
    use crate::graph::undirected::simple_graph::SimpleGraph;
    use crate::service::chromatic_properties::critical_prop::CriticalProperties;
    use crate::service::io::reader_g6::G6Reader;
    use crate::test::test_data::test_data;
    use std::any::Any;
    use crate::service::colour::cvd_dfs::CvdDfsColourizer;
    use crate::service::colour::bfs::BFSColourizer;
    use crate::service::colour::colouriser::Colourizer;

    #[test]
    fn should_be_critical() {
        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_26_CRITICAL_1).unwrap();
        let mut props = CriticalProperties::of_graph(&graph);
        assert_eq!(props.is_critical(), true);
        assert_eq!(props.is_cocritical(), true);
        assert_eq!(props.is_vertex_subcritical(), true);
        assert_eq!(props.is_edge_subcritical(), true);

        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_26_CRITICAL_2).unwrap();
        let mut props = CriticalProperties::of_graph(&graph);
        assert_eq!(props.is_critical(), true);
        assert_eq!(props.is_cocritical(), true);
        assert_eq!(props.is_vertex_subcritical(), true);
        assert_eq!(props.is_edge_subcritical(), true);
    }

    #[test]
    fn should_be_strictly_cocritical() {
        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_26_SCOCRITICAL_1).unwrap();
        let mut props = CriticalProperties::of_graph(&graph);
        assert_eq!(props.is_critical(), false);
        assert_eq!(props.is_cocritical(), true);
        assert_eq!(props.is_vertex_subcritical(), true);
        assert_eq!(props.is_edge_subcritical(), true);

        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_26_SCOCRITICAL_2).unwrap();
        let mut props = CriticalProperties::of_graph(&graph);
        assert_eq!(props.is_critical(), false);
        assert_eq!(props.is_cocritical(), true);
        assert_eq!(props.is_vertex_subcritical(), true);
        assert_eq!(props.is_edge_subcritical(), true);
    }

    #[test]
    fn should_be_acritical() {
        let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_34_STABLE_1).unwrap();
        let mut props = CriticalProperties::of_graph(&graph);
        assert_eq!(props.is_critical(), false);
        assert_eq!(props.is_cocritical(), false);
        assert_eq!(props.is_vertex_subcritical(), false);
        assert_eq!(props.is_edge_subcritical(), false);

        let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_34_STABLE_2).unwrap();
        let mut props = CriticalProperties::of_graph(&graph);
        assert_eq!(props.is_critical(), false);
        assert_eq!(props.is_cocritical(), false);
        assert_eq!(props.is_vertex_subcritical(), false);
        assert_eq!(props.is_edge_subcritical(), false);

        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_30_ACRITICAL_1).unwrap();
        let mut props = CriticalProperties::of_graph(&graph);
        assert_eq!(props.is_critical(), false);
        assert_eq!(props.is_cocritical(), false);
        assert_eq!(props.is_vertex_subcritical(), false);
        assert_eq!(props.is_edge_subcritical(), false);

        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_30_ACRITICAL_2).unwrap();
        let mut props = CriticalProperties::of_graph(&graph);
        assert_eq!(props.is_critical(), false);
        assert_eq!(props.is_cocritical(), false);
        assert_eq!(props.is_vertex_subcritical(), false);
        assert_eq!(props.is_edge_subcritical(), false);
    }

    #[test]
    fn should_be_acritical_cvd_dfs() {
        let colourizer = CvdDfsColourizer::new();
        let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_34_STABLE_1).unwrap();
        let mut props = CriticalProperties::of_graph_with_colourizer(&graph, colourizer);
        assert_eq!(props.is_critical(), false);
        assert_eq!(props.is_cocritical(), false);
        assert_eq!(props.is_vertex_subcritical(), false);
        assert_eq!(props.is_edge_subcritical(), false);

        let colourizer = CvdDfsColourizer::new();
        let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_34_STABLE_2).unwrap();
        let mut props = CriticalProperties::of_graph_with_colourizer(&graph, colourizer);
        assert_eq!(props.is_critical(), false);
        assert_eq!(props.is_cocritical(), false);
        assert_eq!(props.is_vertex_subcritical(), false);
        assert_eq!(props.is_edge_subcritical(), false);

        let colourizer = CvdDfsColourizer::new();
        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_30_ACRITICAL_1).unwrap();
        let mut props = CriticalProperties::of_graph_with_colourizer(&graph, colourizer);
        assert_eq!(props.is_critical(), false);
        assert_eq!(props.is_cocritical(), false);
        assert_eq!(props.is_vertex_subcritical(), false);
        assert_eq!(props.is_edge_subcritical(), false);

        let colourizer = CvdDfsColourizer::new();
        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_30_ACRITICAL_2).unwrap();
        let mut props = CriticalProperties::of_graph_with_colourizer(&graph, colourizer);
        assert_eq!(props.is_critical(), false);
        assert_eq!(props.is_cocritical(), false);
        assert_eq!(props.is_vertex_subcritical(), false);
        assert_eq!(props.is_edge_subcritical(), false);
    }

    #[test]
    fn temp() {
        let colourizer = CvdDfsColourizer::new();
        // let colourizer = BFSColourizer::new();
        let graph: SimpleGraph =
            G6Reader::read_graph(test_data::SNARK_IN_G6_30_ACRITICAL_1).unwrap();
        let mut props = CriticalProperties::of_graph_with_colourizer(&graph, colourizer);
        // let critical = props.is_critical();

        assert_eq!(props.is_critical(), false);
        assert_eq!(props.is_cocritical(), false);
        assert_eq!(props.is_vertex_subcritical(), false);
        assert_eq!(props.is_edge_subcritical(), false);
    }

    #[test]
    fn test() {
        let graph = test_data::get_colorable_graph_20();
        let graph = test_data::get_petersen_graph();

        let graph: SimpleGraph = G6Reader::read_graph("Q?hY@eOGG??B_??@g???T?a??@g").unwrap();
        let graph: SimpleGraph = G6Reader::read_graph("a?gW@eOGG?GA_??_g_?????C?A?C???O???I??@W??W???XO?O??AC?_?_????_??A?????_?????@O????k????o????BG").unwrap();

        let mut critical_properties = CriticalProperties::of_graph(&graph);

        let prop = critical_properties.is_critical();
        println!("prop: {}", prop);
    }
}
