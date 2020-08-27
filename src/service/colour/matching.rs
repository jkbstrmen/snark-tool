use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::simple_graph::SimpleGraph;
use crate::graph::vertex::Vertex;
use crate::service::colour::colouriser::Colourizer;
use crate::service::matching::perfect_matchings::{Matching, MatchingGraph};
use std::cmp::Ordering;
use std::collections::{hash_map, VecDeque};
use std::ops::Deref;
use std::{cmp, collections};

pub struct MatchingColouriser {}

impl Colourizer for MatchingColouriser {
    // TODO - optimize
    fn is_colorable<G: Graph>(graph: &G) -> bool {
        let match_graph = MatchingGraph::from_graph(graph);
        let matchings = match_graph.perfect_matchings();

        for matching in matchings {
            // remove matching from graph
            // TODO - use graph with very fast edge removal
            let mut graph_copy = SimpleGraph::from_graph(graph);
            for edge in matching.edges {
                graph_copy.remove_edge(edge.from(), edge.to());
            }

            // create MatchingGraph from graph
            let match_graph = MatchingGraph::from_graph(&graph_copy);
            let mut cd = CycleDiscovery::new(&match_graph);

            // check if match_graph has odd cycle - if not - colourable true
            let has_odd_cycle = cd.has_odd_cycle();
            if !has_odd_cycle {
                return true;
            }
        }
        false
    }

    fn new() -> Self {
        Self {}
    }
}

///
/// Only for graphs of order at most 2
///
pub struct CycleDiscovery<'a> {
    graph: &'a MatchingGraph,
    visited: Vec<bool>,
    to_visit: Vec<usize>,
    cycle: Vec<usize>,
    vert_iter: hash_map::Iter<'a, usize, Vec<usize>>,
}

impl<'a> CycleDiscovery<'a> {
    pub fn new(graph: &'a MatchingGraph) -> Self {
        let mut visited = vec![false; graph.size()];
        let mut discovery = CycleDiscovery {
            graph,
            visited,
            to_visit: vec![],
            cycle: vec![],
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

    /// (bfs has the same perfomance)
    fn dfs_next(&mut self) -> Option<usize> {
        if let Some(vertex) = self.to_visit.pop() {
            if let Some(neighbors) = self.graph.neighbors(vertex) {
                for neighbor in neighbors {
                    if self.visit(*neighbor) {
                        self.to_visit.push(*neighbor);
                    }
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
        while let Some(component) = self.next_component() {
            if component.len() % 2 == 1 {
                // is odd component

                // check if same number of edges and vertices - if so - it is cycle
                let mut edges_count = 0;
                for vertex in component.iter() {
                    if let Some(neighbors) = self.graph.neighbors(*vertex) {
                        edges_count += neighbors.len();
                    }
                }
                edges_count = edges_count / 2;

                if edges_count == component.len() {
                    // we have cycle and it is of odd size
                    return true;
                }
            }
        }
        false
    }

    fn next_component(&mut self) -> Option<Vec<usize>> {
        self.to_visit = vec![];
        while let Some((vertex, neighbors)) = self.vert_iter.next() {
            if self.visited[*vertex] {
                continue;
            }
            self.to_visit.push(*vertex);
            self.visit(*vertex);
            let mut chain = vec![];
            while let Some(dfs_next_vertex) = self.dfs_next() {
                chain.push(dfs_next_vertex);
            }
            return Some(chain);
        }
        None
    }

    fn next_non_visited_vertex(&mut self) -> Option<usize> {
        while let Some(vertex) = self.vert_iter.next() {
            if !self.visited[*vertex.0] {
                return Some(*vertex.0);
            }
        }
        None
    }
}
