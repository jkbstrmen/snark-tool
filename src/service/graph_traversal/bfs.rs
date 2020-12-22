use crate::graph::temp_graph::TempGraph;
use std::collections::VecDeque;

// TODO - deduplicate with BfsGraph from perfect_matchings and cvd and put to own file
// TODO - replace TempGraph with Graph

pub struct BfsOfGraph<'a, G: TempGraph> {
    graph: &'a G,
    visited: Vec<bool>,
    to_visit: VecDeque<usize>,
}

impl<'a, G: TempGraph> BfsOfGraph<'a, G> {
    pub fn new(graph: &'a G, start: usize) -> Self {
        let visited = vec![false; graph.size()];
        let mut to_visit = VecDeque::new();
        to_visit.push_back(start);

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
        let old_val = self.visited[vertex];
        self.visited[vertex] = true;
        !old_val
    }

    pub fn bfs_next(&mut self) -> Option<usize> {
        if let Some(vertex) = self.to_visit.pop_front() {
            for neighbor in self.graph.neighbors_of_vertex(vertex) {
                if self.visit(neighbor) {
                    self.to_visit.push_back(neighbor);
                }
            }
            return Some(vertex);
        }
        None
    }
}
