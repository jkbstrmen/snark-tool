use crate::graph::graph::Graph;
use crate::graph::temp_graph::TempGraph;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct CycleDiscoveryVertex {
    visited: bool,
    processed: bool,
    distance_from_root: usize,
}

impl CycleDiscoveryVertex {
    pub fn new() -> Self {
        CycleDiscoveryVertex {
            visited: false,
            processed: false,
            distance_from_root: 0,
        }
    }
}

pub struct BFSCyclehDiscovery<'a, G: Graph> {
    graph: &'a G,
    visited: Vec<CycleDiscoveryVertex>,
    to_visit: VecDeque<usize>,
}

impl<'a, G: Graph> BFSCyclehDiscovery<'a, G> {
    pub fn new(graph: &'a G, start: usize) -> Self {
        let visited = vec![CycleDiscoveryVertex::new(); graph.size()];
        let mut to_visit = VecDeque::new();
        to_visit.push_back(start);

        let mut cd = Self {
            graph,
            visited,
            to_visit,
        };
        cd.visit(start);
        cd.set_distance_from_root(start, 0);
        cd
    }

    ///
    /// if true, visited for the first time
    ///
    fn visit(&mut self, vertex: usize) -> bool {
        let old_val = self.visited[vertex].visited;
        self.visited[vertex].visited = true;
        !old_val
    }

    fn process(&mut self, vertex: usize) {
        self.visited[vertex].processed = true;
    }

    fn processed(&self, vertex: usize) -> bool {
        self.visited[vertex].processed
    }

    fn set_distance_from_root(&mut self, vertex: usize, distance: usize) {
        self.visited[vertex].distance_from_root = distance;
    }

    fn distance_from_root(&self, vertex: usize) -> usize {
        self.visited[vertex].distance_from_root
    }

    ///
    /// works correctly for first found cycle
    ///
    pub fn length_of_next_cycle(&mut self) -> usize {
        while let Some(vertex) = self.to_visit.pop_front() {
            let distance_from_root = self.visited[vertex].distance_from_root;
            for neighbor in self.graph.neighbors_of_vertex(vertex) {
                if self.processed(neighbor) {
                    continue;
                }
                if self.visit(neighbor) {
                    self.to_visit.push_back(neighbor);
                } else {
                    return self.distance_from_root(vertex) + self.distance_from_root(neighbor) + 1;
                }
                self.set_distance_from_root(neighbor, distance_from_root + 1);
            }
            self.process(vertex);
        }
        0
    }
}
