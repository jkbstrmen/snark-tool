use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::service::colour::colouriser::Colouriser;
use crate::service::matching::perfect_matchings::{
    Matching, MatchingGraph, MatchingGraphVerticesIter,
};

// TODO - add to procedures

pub struct MatchingColouriser {}

impl Colouriser for MatchingColouriser {
    fn is_colorable<G: Graph>(graph: &G) -> bool {
        let mut match_graph = MatchingGraph::from_graph(graph);
        let matchings = match_graph.perfect_matchings();
        let is_col = Self::is_col(graph, &matchings);
        is_col
    }

    fn new() -> Self {
        Self {}
    }
}

impl MatchingColouriser {
    ///
    /// if exist matching M such that graph minus M doesn't contain odd length cycle -> graph is
    /// colourable
    ///
    fn is_col<G: Graph>(graph: &G, matchings: &Vec<Matching>) -> bool {
        let mut local_graph = MatchingGraph::from_graph(graph);
        for matching in matchings {
            // remove matching from graph
            for edge in matching.edges.iter() {
                local_graph.remove_edge(edge.from(), edge.to());
            }
            let mut cd = CycleDiscovery::new(&local_graph);

            // check if match_graph has odd cycle - if not - colourable true
            let has_odd_cycle = cd.has_odd_cycle();
            if !has_odd_cycle {
                return true;
            }
            // recover removed edges for next iteration
            for edge in matching.edges.iter() {
                // graph_copy.remove_edge(edge.from(), edge.to());
                local_graph.add_edge(edge.from(), edge.to());
            }
        }
        false
    }

    //     TODO - maybe try to find colouring by comparing edge sets of each pair of perfect matchings of graph
}

///
/// Only for graphs of order at most 2
///
pub struct CycleDiscovery<'a> {
    graph: &'a MatchingGraph,
    visited: Vec<bool>,
    to_visit: Vec<usize>,
    vert_iter: MatchingGraphVerticesIter<'a>,
}

impl<'a> CycleDiscovery<'a> {
    pub fn new(graph: &'a MatchingGraph) -> Self {
        let visited = vec![false; graph.max_vertex_index()];
        let discovery = CycleDiscovery {
            graph,
            visited,
            to_visit: vec![],
            vert_iter: graph.vertices(),
        };
        discovery
    }

    ///
    /// if true, visited for the first time
    ///
    fn visit(&mut self, vertex: usize) -> bool {
        let old_val = self.visited[vertex];
        self.visited[vertex] = true;
        !old_val
    }

    ///
    /// (bfs has the same perfomance)
    ///
    fn dfs_next(&mut self) -> Option<usize> {
        if let Some(vertex) = self.to_visit.pop() {
            for neighbor in self.graph.neighbors(vertex) {
                if self.visit(*neighbor) {
                    self.to_visit.push(*neighbor);
                }
            }

            return Some(vertex);
        }
        None
    }

    ///
    /// Works only if self.graph is graph of order 2
    ///
    pub fn has_odd_cycle(&mut self) -> bool {
        // clear internal data
        self.visited = vec![false; self.graph.max_vertex_index()];
        while let Some(component) = self.next_component() {
            if component.len() % 2 == 1 {
                // is odd component
                if self.is_cycle(&component) {
                    return true;
                }
            }
        }
        false
    }

    ///
    /// Works only if self.graph is graph of order 2
    ///
    fn is_cycle(&self, component: &Vec<usize>) -> bool {
        // check if same number of edges and vertices - if so - it is cycle
        let mut edges_count = 0;
        for vertex in component.iter() {
            let neighbors = self.graph.neighbors(*vertex);
            edges_count += neighbors.len();
        }
        edges_count = edges_count / 2;

        if edges_count == component.len() {
            return true;
        }
        false
    }

    fn next_component(&mut self) -> Option<Vec<usize>> {
        self.to_visit = vec![];
        while let Some(vertex) = self.vert_iter.next() {
            if self.visited[*vertex.index()] {
                continue;
            }
            self.to_visit.push(*vertex.index());
            self.visit(*vertex.index());
            let mut chain = vec![];
            while let Some(dfs_next_vertex) = self.dfs_next() {
                chain.push(dfs_next_vertex);
            }
            return Some(chain);
        }
        None
    }

    // fn next_non_visited_vertex(&mut self) -> Option<usize> {
    //     while let Some(vertex) = self.vert_iter.next() {
    //         if !self.visited[*vertex.index()] {
    //             return Some(*vertex.index());
    //         }
    //     }
    //     None
    // }

    pub fn cycles(&mut self) -> Vec<Vec<usize>> {
        // clear internal data
        self.visited = vec![false; self.graph.max_vertex_index()];
        let mut cycles = vec![];
        while let Some(component) = self.next_component() {
            if self.is_cycle(&component) {
                cycles.push(component);
            }
        }
        cycles
    }
}
