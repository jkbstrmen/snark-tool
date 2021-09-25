use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::simple_graph::graph::SimpleGraph;
use crate::service::colour::colouriser::Colouriser;
use crate::service::colour::recursive::dfs_improved::DFSColourizer;
use crate::service::constructions::dot_product::DotProducts;
use crate::service::constructions::i_extension::{i_extension, IExtensions};
use crate::service::constructions::y_extension::{y_extension, YExtensions};
use crate::service::io::reader_g6::G6Reader;
use crate::tests::test_data::test_data;

#[test]
fn dot_product_test() {
    let graph_g = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();
    let graph_h = G6Reader::<SimpleGraph>::read_graph(test_data::SNARK_IN_G6_10_PETERSEN).unwrap();

    let mut dot_product = DotProducts::new(&graph_g, &graph_h);
    let gh = dot_product.next().unwrap();

    assert_eq!(gh.size(), graph_g.size() + graph_h.size() - 2);
    let colourable = DFSColourizer::is_colorable(&gh);
    assert_eq!(colourable, false);
    assert_eq!(gh.has_edge(5, 13), true);
    assert_eq!(gh.has_edge(1, 11), true);
    assert_eq!(gh.has_edge(4, 16), true);
    assert_eq!(gh.has_edge(0, 14), true);

    let edges_check = vec![
        (0, 6),
        (0, 8),
        (0, 14),
        (1, 6),
        (1, 9),
        (1, 11),
        (2, 4),
        (2, 7),
        (2, 9),
        (3, 5),
        (3, 7),
        (3, 8),
        (4, 5),
        (4, 16),
        (5, 13),
        (6, 7),
        (8, 9),
        (10, 13),
        (10, 14),
        (10, 17),
        (11, 15),
        (11, 17),
        (12, 13),
        (12, 15),
        (12, 16),
        (14, 15),
        (16, 17),
    ];
    let mut edges = vec![];
    for edge in gh.edges() {
        edges.push((edge.from(), edge.to()));
    }
    assert_eq!(edges_check, edges);

    while let Some(dot_product) = dot_product.next() {
        let colourable = DFSColourizer::is_colorable(&dot_product);
        assert_eq!(colourable, false);
    }
}

#[test]
fn i_extension_test() {
    let graph = test_data::get_petersen_graph();
    let first_edge = UndirectedEdge::new(0, 4);
    let second_edge = UndirectedEdge::new(3, 5);
    let i_extended = i_extension(&graph, &first_edge, &second_edge);

    assert_eq!(i_extended.size(), graph.size() + 2);

    let edges_check = vec![
        (0, 6),
        (0, 8),
        (0, 10),
        (1, 5),
        (1, 6),
        (1, 9),
        (2, 4),
        (2, 7),
        (2, 9),
        (3, 7),
        (3, 8),
        (3, 11),
        (4, 5),
        (4, 10),
        (5, 11),
        (6, 7),
        (8, 9),
        (10, 11),
    ];
    let mut edges = vec![];
    for edge in i_extended.edges() {
        edges.push((edge.from(), edge.to()));
    }
    assert_eq!(edges, edges_check);
}

#[test]
fn i_extension_iterator_test() {
    let graph = test_data::get_falcon_graph();
    let colouriser = DFSColourizer::new();
    let i_extensions = IExtensions::new(&graph, &colouriser);

    let mut counter = 0;
    for i_extension in i_extensions {
        let colourable = DFSColourizer::is_colorable(&i_extension);
        assert_eq!(colourable, false);
        counter += 1;
    }
    assert_eq!(counter, 1431);
}

#[test]
fn y_extension_test() {
    let graph = test_data::get_petersen_graph();

    let first_edge = UndirectedEdge::new(0, 4);
    let second_edge = UndirectedEdge::new(3, 5);
    let third_edge = UndirectedEdge::new(8, 9);
    let extended = y_extension(&graph, &first_edge, &second_edge, &third_edge);

    assert_eq!(extended.size(), graph.size() + 4);
}

#[test]
fn y_extension_iterator_test() {
    // with bigger graph test takes longer to finish
    // let graph = test_data::get_falcon_graph();
    let graph = test_data::get_petersen_graph();

    let colouriser = DFSColourizer::new();
    let mut y_extensions = YExtensions::new(&graph, &colouriser);
    let mut counter = 0;
    while let Some(extended) = y_extensions.next() {
        assert_eq!(extended.size(), graph.size() + 4);
        let colourable = DFSColourizer::is_colorable(&extended);
        assert_eq!(colourable, false);
        counter += 1;
    }
    // for falcon graph
    // assert_eq!(counter, 21516);
    // for petersen graph
    assert_eq!(counter, 10);
}

#[test]
fn two_i_extension_test() {
    // TODO
}

use crate::service::constructions::isomorphism::is_isomorphic;

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
