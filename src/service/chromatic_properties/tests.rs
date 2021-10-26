use crate::graph::undirected::simple_graph::graph::SimpleGraph;
use crate::service::chromatic_properties::critical_prop::CriticalPropertiesSolver;
use crate::service::chromatic_properties::CriticalProperties;
use crate::service::colour::colouriser::Colouriser;
use crate::service::colour::cvd::cvd_dfs::CvdDfsColourizer;
use crate::service::colour::matchings::matching_col::MatchingColouriser;
use crate::service::io::reader_g6::G6Reader;
use crate::tests::test_data::test_data;

#[test]
fn should_be_critical() {
    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_26_CRITICAL_1).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph(&graph);
    assert_eq!(props.is_critical(), true);
    assert_eq!(props.is_cocritical(), true);
    assert_eq!(props.is_vertex_subcritical(), true);
    assert_eq!(props.is_edge_subcritical(), true);

    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_26_CRITICAL_2).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph(&graph);
    assert_eq!(props.is_critical(), true);
    assert_eq!(props.is_cocritical(), true);
    assert_eq!(props.is_vertex_subcritical(), true);
    assert_eq!(props.is_edge_subcritical(), true);
}

#[test]
fn should_be_strictly_cocritical() {
    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_26_SCOCRITICAL_1).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph(&graph);
    assert_eq!(props.is_critical(), false);
    assert_eq!(props.is_cocritical(), true);
    assert_eq!(props.is_vertex_subcritical(), true);
    assert_eq!(props.is_edge_subcritical(), true);

    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_26_SCOCRITICAL_2).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph(&graph);
    assert_eq!(props.is_critical(), false);
    assert_eq!(props.is_cocritical(), true);
    assert_eq!(props.is_vertex_subcritical(), true);
    assert_eq!(props.is_edge_subcritical(), true);
}

#[test]
fn should_be_acritical() {
    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_34_STABLE_1).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph(&graph);
    assert_eq!(props.is_critical(), false);
    assert_eq!(props.is_cocritical(), false);
    assert_eq!(props.is_vertex_subcritical(), false);
    assert_eq!(props.is_edge_subcritical(), false);

    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_34_STABLE_2).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph(&graph);
    assert_eq!(props.is_critical(), false);
    assert_eq!(props.is_cocritical(), false);
    assert_eq!(props.is_vertex_subcritical(), false);
    assert_eq!(props.is_edge_subcritical(), false);

    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_30_ACRITICAL_1).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph(&graph);
    assert_eq!(props.is_critical(), false);
    assert_eq!(props.is_cocritical(), false);
    assert_eq!(props.is_vertex_subcritical(), false);
    assert_eq!(props.is_edge_subcritical(), false);

    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_30_ACRITICAL_2).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph(&graph);
    assert_eq!(props.is_critical(), false);
    assert_eq!(props.is_cocritical(), false);
    assert_eq!(props.is_vertex_subcritical(), false);
    assert_eq!(props.is_edge_subcritical(), false);
}

#[test]
fn should_be_acritical_cvd_dfs() {
    let colourizer = CvdDfsColourizer::new();
    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_34_STABLE_1).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph_with_colourizer(&graph, colourizer);
    assert_eq!(props.is_critical(), false);
    assert_eq!(props.is_cocritical(), false);
    assert_eq!(props.is_vertex_subcritical(), false);
    assert_eq!(props.is_edge_subcritical(), false);

    let colourizer = CvdDfsColourizer::new();
    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_34_STABLE_2).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph_with_colourizer(&graph, colourizer);
    assert_eq!(props.is_critical(), false);
    assert_eq!(props.is_cocritical(), false);
    assert_eq!(props.is_vertex_subcritical(), false);
    assert_eq!(props.is_edge_subcritical(), false);

    let colourizer = CvdDfsColourizer::new();
    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_30_ACRITICAL_1).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph_with_colourizer(&graph, colourizer);
    assert_eq!(props.is_critical(), false);
    assert_eq!(props.is_cocritical(), false);
    assert_eq!(props.is_vertex_subcritical(), false);
    assert_eq!(props.is_edge_subcritical(), false);

    let colourizer = CvdDfsColourizer::new();
    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_30_ACRITICAL_2).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph_with_colourizer(&graph, colourizer);
    assert_eq!(props.is_critical(), false);
    assert_eq!(props.is_cocritical(), false);
    assert_eq!(props.is_vertex_subcritical(), false);
    assert_eq!(props.is_edge_subcritical(), false);
}

#[test]
fn should_be_critical_matchings() {
    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_26_CRITICAL_1).unwrap();
    let mut props =
        CriticalPropertiesSolver::of_graph_with_colourizer(&graph, MatchingColouriser::new());
    assert_eq!(props.is_critical(), true);
    assert_eq!(props.is_cocritical(), true);
    assert_eq!(props.is_vertex_subcritical(), true);
    assert_eq!(props.is_edge_subcritical(), true);

    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_26_CRITICAL_2).unwrap();
    let mut props =
        CriticalPropertiesSolver::of_graph_with_colourizer(&graph, MatchingColouriser::new());
    assert_eq!(props.is_critical(), true);
    assert_eq!(props.is_cocritical(), true);
    assert_eq!(props.is_vertex_subcritical(), true);
    assert_eq!(props.is_edge_subcritical(), true);
}

#[test]
fn should_be_acritical_matching() {
    let colourizer = MatchingColouriser::new();
    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_34_STABLE_1).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph_with_colourizer(&graph, colourizer);
    assert_eq!(props.is_critical(), false);
    assert_eq!(props.is_cocritical(), false);
    assert_eq!(props.is_vertex_subcritical(), false);
    assert_eq!(props.is_edge_subcritical(), false);

    let colourizer = MatchingColouriser::new();
    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_34_STABLE_2).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph_with_colourizer(&graph, colourizer);
    assert_eq!(props.is_critical(), false);
    assert_eq!(props.is_cocritical(), false);
    assert_eq!(props.is_vertex_subcritical(), false);
    assert_eq!(props.is_edge_subcritical(), false);

    let colourizer = MatchingColouriser::new();
    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_30_ACRITICAL_1).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph_with_colourizer(&graph, colourizer);
    assert_eq!(props.is_critical(), false);
    assert_eq!(props.is_cocritical(), false);
    assert_eq!(props.is_vertex_subcritical(), false);
    assert_eq!(props.is_edge_subcritical(), false);

    let colourizer = MatchingColouriser::new();
    let graph: SimpleGraph = G6Reader::read_graph(test_data::SNARK_IN_G6_30_ACRITICAL_2).unwrap();
    let mut props = CriticalPropertiesSolver::of_graph_with_colourizer(&graph, colourizer);
    assert_eq!(props.is_critical(), false);
    assert_eq!(props.is_cocritical(), false);
    assert_eq!(props.is_vertex_subcritical(), false);
    assert_eq!(props.is_edge_subcritical(), false);
}
