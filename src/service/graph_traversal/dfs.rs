use crate::graph::graph::Graph;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct DfsVertex {
    index: usize,
    visited: bool,
    discovered_from: usize,
    distance_from_root: usize,
}

impl DfsVertex {
    pub fn new(
        index: usize,
        visited: bool,
        discovered_from: usize,
        distance_from_root: usize,
    ) -> Self {
        DfsVertex {
            index,
            visited,
            discovered_from,
            distance_from_root,
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
}

#[derive(Debug, Clone)]
pub struct DfsOfGraph<'a, G: Graph + Clone> {
    graph: &'a G,
    visited: Vec<Option<DfsVertex>>,
    to_visit: VecDeque<DfsVertex>,
}

impl<'a, G: Graph + Clone> DfsOfGraph<'a, G> {
    pub fn new(graph: &'a G, start: usize) -> Self {
        let visited = vec![None; graph.size()];
        let mut to_visit = VecDeque::new();
        to_visit.push_back(DfsVertex::new(start, false, start, 0));

        let mut bfs = Self {
            graph,
            visited,
            to_visit,
        };
        let start_vertex = DfsVertex::new(start, false, start, 0);
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
    fn visit_from(&mut self, vertex: usize, visited_from: usize) -> bool {
        if let Some(from) = &self.visited[visited_from] {
            let visited_vertex =
                DfsVertex::new(vertex, true, from.index, from.distance_from_root + 1);
            self.visited[vertex] = Some(visited_vertex);
        }
        true
    }

    pub fn next(&mut self) -> Option<DfsVertex> {
        if let Some(vertex) = self.to_visit.pop_back() {
            if let Some(vert) = &self.visited[vertex.index] {
                if vert.visited {
                    return self.next();
                }
            }

            for neighbor in self.graph.neighbors_of_vertex(vertex.index) {
                if self.visited[neighbor].is_none() {
                    self.to_visit.push_back(DfsVertex::new(
                        neighbor,
                        false,
                        vertex.index,
                        vertex.distance_from_root + 1,
                    ));
                }
            }
            self.visit_from(vertex.index, vertex.discovered_from);
            return Some(vertex);
        }
        None
    }

    pub fn visited_vertex(&self, index: usize) -> Option<&DfsVertex> {
        let vert = self.visited.get(index);
        if let Some(result) = vert {
            return result.as_ref();
        }
        None
    }
}
