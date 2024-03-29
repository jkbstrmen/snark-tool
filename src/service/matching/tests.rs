use crate::service::matching::perfect_matchings::{MatchingGraph, MatchingVertex};
use crate::tests::test_data::test_data;

#[test]
fn should_create_from_graph_mg() {
    let s_graph = test_data::get_petersen_graph();
    let m_graph = MatchingGraph::from_graph(&s_graph);

    assert_edges_petersen(&m_graph);
}

fn assert_edges_petersen(graph: &MatchingGraph) {
    assert_eq!(graph.has_edge(0, 4), true);
    assert_eq!(graph.has_edge(0, 6), true);
    assert_eq!(graph.has_edge(0, 8), true);
    assert_eq!(graph.has_edge(1, 5), true);
    assert_eq!(graph.has_edge(1, 6), true);
    assert_eq!(graph.has_edge(1, 9), true);
    assert_eq!(graph.has_edge(2, 4), true);
    assert_eq!(graph.has_edge(2, 7), true);
    assert_eq!(graph.has_edge(2, 9), true);
    assert_eq!(graph.has_edge(3, 5), true);
    assert_eq!(graph.has_edge(3, 7), true);
    assert_eq!(graph.has_edge(3, 8), true);
    assert_eq!(graph.has_edge(4, 5), true);
    assert_eq!(graph.has_edge(6, 7), true);
    assert_eq!(graph.has_edge(8, 9), true);

    assert_eq!(graph.has_edge(0, 5), false);
    assert_eq!(graph.has_edge(2, 8), false);
}

#[test]
fn should_create_and_modify_graph_mg() {
    let mut graph = MatchingGraph::with_capacity(10);
    graph.add_edge(0, 4);
    graph.add_edge(0, 6);
    graph.add_edge(0, 8);
    graph.add_edge(1, 5);
    graph.add_edge(1, 6);
    graph.add_edge(1, 9);
    graph.add_edge(2, 4);
    graph.add_edge(2, 7);
    graph.add_edge(2, 9);
    graph.add_edge(3, 5);
    graph.add_edge(3, 7);
    graph.add_edge(3, 8);
    graph.add_edge(4, 5);
    graph.add_edge(6, 7);
    graph.add_edge(8, 9);

    assert_edges_petersen(&graph);

    assert_eq!(graph.size(), 10);
    graph.remove_vertex(0);
    assert_eq!(graph.has_edge(0, 4), false);
    assert_eq!(graph.has_edge(0, 6), false);
    assert_eq!(graph.has_edge(0, 8), false);
    assert_eq!(graph.size(), 9);
    graph.add_vertex(MatchingVertex::new(0));
    assert_eq!(graph.has_edge(0, 4), false);
    assert_eq!(graph.has_edge(0, 6), false);
    assert_eq!(graph.has_edge(0, 8), false);
    assert_eq!(graph.size(), 10);
    graph.add_edge(0, 4);
    graph.add_edge(0, 6);
    graph.add_edge(0, 8);

    assert_edges_petersen(&graph);
    asser_graph_simpleness(&graph);

    graph.remove_vertex(0);
    graph.remove_vertex(1);

    graph.add_edge(0, 4);
    graph.add_edge(0, 6);
    graph.add_edge(0, 8);
    graph.add_edge(1, 5);
    graph.add_edge(1, 6);
    graph.add_edge(1, 9);

    asser_graph_simpleness(&graph);
    assert_edges_petersen(&graph);
}

fn asser_graph_simpleness(graph: &MatchingGraph) {
    for vertex in graph.vertices() {
        let mut neighbors = vec![];
        for neighbor in vertex.neighbors() {
            neighbors.push(neighbor);
        }
        let len_before = neighbors.len();
        neighbors.sort();
        neighbors.dedup();
        assert_eq!(len_before, neighbors.len());
    }
}

#[test]
fn should_add_and_remove_edges_mg() {
    let s_graph = test_data::get_petersen_graph();
    let mut m_graph = MatchingGraph::from_graph(&s_graph);

    assert_edges_petersen(&m_graph);

    m_graph.remove_edge(0, 4);
    assert_eq!(m_graph.has_edge(0, 4), false);
    m_graph.add_edge(0, 4);
    assert_eq!(m_graph.has_edge(0, 4), true);

    m_graph.remove_edge(1, 6);
    assert_eq!(m_graph.has_edge(1, 6), false);
    m_graph.add_edge(1, 6);
    assert_eq!(m_graph.has_edge(1, 6), true);

    m_graph.add_edge(1, 6);
    asser_graph_simpleness(&m_graph);
}

use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::service::matching::perfect_matchings::BfsGraph;

#[test]
fn should_traverse_using_bfs() {
    let graph = test_data::get_petersen_graph();

    let mut match_graph = MatchingGraph::new();
    for edge in graph.edges() {
        match_graph.add_edge(edge.from(), edge.to());
    }
    let bfs_vertices = vec![0, 4, 6, 8, 2, 5, 1, 7, 3, 9];
    let mut index = 0;
    let mut bfs_graph = BfsGraph::new(&match_graph, 0);
    while let Some(vertex) = bfs_graph.bfs_next() {
        assert_eq!(bfs_vertices[index], vertex);
        index += 1;
    }
}

#[test]
fn should_not_have_odd_size_component() {
    let mut match_graph = MatchingGraph::new();
    let has = match_graph.has_odd_size_component();
    assert_eq!(has, false);

    add_edges_to_graph(&mut match_graph, test_data::first_even_component());
    let has = match_graph.has_odd_size_component();
    assert_eq!(has, false);

    let mut match_graph = MatchingGraph::new();
    add_edges_to_graph(&mut match_graph, test_data::second_even_component());
    let has = match_graph.has_odd_size_component();
    assert_eq!(has, false);

    let mut match_graph = MatchingGraph::new();
    add_edges_to_graph(&mut match_graph, test_data::third_even_component_petersen());
    let has = match_graph.has_odd_size_component();
    assert_eq!(has, false);

    add_edges_to_graph(&mut match_graph, test_data::second_even_component());
    add_edges_to_graph(&mut match_graph, test_data::first_even_component());
    let has = match_graph.has_odd_size_component();
    assert_eq!(has, false);

    add_edges_to_graph(&mut match_graph, test_data::first_odd_component());
    let has = match_graph.has_odd_size_component();
    assert_eq!(has, true);

    let mut match_graph = MatchingGraph::new();
    add_edges_to_graph(&mut match_graph, test_data::second_odd_component());
    match_graph.remove_vertex(16);
    let has = match_graph.has_odd_size_component();
    assert_eq!(has, false);
}

#[test]
fn should_have_odd_size_component() {
    let mut match_graph = MatchingGraph::new();
    add_edges_to_graph(&mut match_graph, test_data::first_odd_component());
    let has = match_graph.has_odd_size_component();
    assert_eq!(has, true);

    add_edges_to_graph(&mut match_graph, test_data::first_even_component());
    let has = match_graph.has_odd_size_component();
    assert_eq!(has, true);

    add_edges_to_graph(&mut match_graph, test_data::second_even_component());
    let has = match_graph.has_odd_size_component();
    assert_eq!(has, true);

    let mut match_graph = MatchingGraph::new();
    add_edges_to_graph(&mut match_graph, test_data::first_even_component());
    let has = match_graph.has_odd_size_component();
    assert_eq!(has, false);

    add_edges_to_graph(&mut match_graph, test_data::second_odd_component());
    let has = match_graph.has_odd_size_component();
    assert_eq!(has, true);

    let mut match_graph = MatchingGraph::new();
    match_graph.create_vertex_if_not_exists(0);
    let has = match_graph.has_odd_size_component();
    assert_eq!(has, true);
    match_graph.create_vertex_if_not_exists(1);
    let has = match_graph.has_odd_size_component();
    assert_eq!(has, true);
}

fn add_edges_to_graph(graph: &mut MatchingGraph, edges: Vec<UndirectedEdge>) {
    for edge in edges {
        graph.add_edge(edge.from(), edge.to());
    }
}

#[test]
fn should_find_all_perfect_matchings() {
    let mut graph = MatchingGraph::from_graph(&test_data::get_petersen_graph());
    let mut matchings = graph.perfect_matchings();
    let petersens_matchings = test_data::petersens_matchings();

    for matching in matchings.iter_mut() {
        matching.edges.sort();
    }
    matchings.sort();

    assert_eq!(petersens_matchings[0], matchings[0]);
    assert_eq!(petersens_matchings[1], matchings[1]);
    assert_eq!(petersens_matchings[2], matchings[2]);
    assert_eq!(petersens_matchings[3], matchings[3]);
    assert_eq!(petersens_matchings[4], matchings[4]);
    assert_eq!(petersens_matchings[5], matchings[5]);
}
