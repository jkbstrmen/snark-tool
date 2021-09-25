use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::service::colour::colouriser::Colouriser;
use crate::service::matching::perfect_matchings::{Matching, MatchingGraph};

///
/// MatchingColouriser2
/// finds all perfect matchings and then compares each pair if matchings are disjoint
///
/// TODO maybe try to find colouring by comparing edge sets of each pair of perfect matchings of graph
///
#[derive(Debug, Clone)]
pub struct MatchingColouriser2 {}

impl Colouriser for MatchingColouriser2 {
    fn is_colorable<G: Graph>(graph: &G) -> bool {
        let mut match_graph = MatchingGraph::from_graph(graph);
        let matchings = match_graph.perfect_matchings();
        let is_col = Self::is_col(&matchings);
        is_col
    }

    fn new() -> Self {
        Self {}
    }
}

impl MatchingColouriser2 {
    ///
    /// if exist matching M such that graph minus M doesn't contain odd length cycle -> graph is
    /// colourable
    ///
    fn is_col(matchings: &Vec<Matching>) -> bool {
        let mut i = 0;
        for first in matchings.iter() {
            i = i + 1;
            for j in i..matchings.len() {
                let second = &matchings[j];
                if Self::are_disjoint(first, second) {
                    return true;
                }
            }
        }
        false
    }

    fn are_disjoint(first: &Matching, second: &Matching) -> bool {
        for edge in first.edges.iter() {
            if Self::has_edge(second, edge) {
                return false;
            }
        }
        true
    }

    fn has_edge(matching: &Matching, edge: &UndirectedEdge) -> bool {
        for matching_edge in matching.edges.iter() {
            if edge == matching_edge {
                return true;
            }
        }
        false
    }
}
