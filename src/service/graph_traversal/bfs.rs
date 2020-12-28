use crate::graph::graph::Graph;
use std::collections::VecDeque;

// TODO - deduplicate with BfsGraph from perfect_matchings and cvd and put to own file

#[derive(Debug, Clone)]
pub struct BfsVertex {
    index: usize,
    visited: bool,
    discovered_from: usize,
    distance_from_root: usize,
}

impl BfsVertex {
    pub fn new(
        index: usize,
        visited: bool,
        discovered_from: usize,
        distance_from_root: usize,
    ) -> Self {
        BfsVertex {
            index,
            visited,
            discovered_from,
            distance_from_root,
        }
    }

    pub fn default() -> Self {
        BfsVertex {
            index: 0,
            visited: false,
            discovered_from: 0,
            distance_from_root: 0,
        }
    }

    /// GETTERS
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn visited(&self) -> bool {
        self.visited
    }
    pub fn discovered_from(&self) -> usize {
        self.discovered_from
    }
    pub fn distance_from_root(&self) -> usize {
        self.distance_from_root
    }

    /// SETTERS
    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }
    pub fn set_visited(&mut self, visited: bool) {
        self.visited = visited;
    }
    pub fn set_discovered_from(&mut self, discovered_from: usize) {
        self.discovered_from = discovered_from;
    }
}

pub struct BfsOfGraph<'a, G: Graph> {
    graph: &'a G,
    // visited: Vec<bool>,
    // to_visit: VecDeque<usize>,
    visited: Vec<BfsVertex>,
    to_visit: VecDeque<BfsVertex>,
}

impl<'a, G: Graph> BfsOfGraph<'a, G> {
    pub fn new(graph: &'a G, start: usize) -> Self {
        let visited = vec![BfsVertex::default(); graph.size()];
        let mut to_visit = VecDeque::new();
        to_visit.push_back(BfsVertex::new(start, true, start, 0));

        let mut bfs = Self {
            graph,
            visited,
            to_visit,
        };
        bfs.visit(start);
        bfs
    }

    ///
    /// if true, visited for the first time
    ///
    fn visit(&mut self, vertex: usize) -> bool {
        let old_val = self.visited[vertex].visited;
        self.visited[vertex].visited = true;
        !old_val
    }

    fn visit_from(&mut self, vertex: usize, visited_from: &BfsVertex) -> bool {
        let already_visited = self.visited[vertex].visited;
        if !already_visited {
            self.visited[vertex].index = vertex;
            self.visited[vertex].visited = true;
            self.visited[vertex].discovered_from = visited_from.index;
            self.visited[vertex].distance_from_root = visited_from.distance_from_root + 1;
        }
        !already_visited
    }

    pub fn next(&mut self) -> Option<BfsVertex> {
        if let Some(vertex) = self.to_visit.pop_front() {
            for neighbor in self.graph.neighbors_of_vertex(vertex.index) {
                if self.visit_from(neighbor, &vertex) {
                    self.to_visit.push_back(BfsVertex::new(
                        neighbor,
                        false,
                        vertex.index,
                        vertex.distance_from_root + 1,
                    ));
                }
            }
            return Some(vertex);
        }
        None
    }
}
