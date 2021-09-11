use crate::graph::graph::Graph;
use std::collections::VecDeque;

// TODO - deduplicate with BfsGraph from perfect_matchings and cvd

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
    visited: Vec<Option<BfsVertex>>,
    to_visit: VecDeque<BfsVertex>,
    discovery_order: Vec<usize>,
}

impl<'a, G: Graph> BfsOfGraph<'a, G> {
    pub fn new(graph: &'a G, start: usize) -> Self {
        let visited = vec![None; graph.size()];
        let mut to_visit = VecDeque::new();
        to_visit.push_back(BfsVertex::new(start, true, start, 0));

        let mut bfs = Self {
            graph,
            visited,
            to_visit,
            discovery_order: vec![],
        };
        let start_vertex = BfsVertex::new(start, true, start, 0);
        bfs.visited[start] = Some(start_vertex);
        bfs
    }

    pub unsafe fn new_from_raw_ptr(graph: *const G, start: usize) -> Self {
        let graph_ref = &(*graph);
        Self::new(graph_ref, start)
    }

    ///
    /// if true, visited for the first time
    ///
    fn visit_from(&mut self, vertex: usize, visited_from: &BfsVertex) -> bool {
        let vert = &self.visited[vertex];
        let mut already_visited = false;
        if vert.is_some() && vert.as_ref().unwrap().visited() {
            already_visited = true;
        }
        if !already_visited {
            let visited_vertex = BfsVertex::new(
                vertex,
                true,
                visited_from.index,
                visited_from.distance_from_root + 1,
            );
            self.visited[vertex] = Some(visited_vertex);
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
            self.discovery_order.push(vertex.index);
            return Some(vertex);
        }
        None
    }

    pub fn back(&mut self) {
        if let Some(last) = self.discovery_order.pop() {
            if let Some(mut last_vertex) = self.visited[last].clone() {
                last_vertex.visited = false;
                self.to_visit.push_front(last_vertex);
            }
        }
    }

    pub fn visited_vertex(&self, index: usize) -> Option<&BfsVertex> {
        let vert = self.visited.get(index);
        if let Some(result) = vert {
            return result.as_ref();
        }
        None
    }

    pub fn discovery_order(&self) -> &Vec<usize> {
        &self.discovery_order
    }
}
