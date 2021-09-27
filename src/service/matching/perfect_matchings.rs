use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use std::collections::VecDeque;
use std::slice;

// TODO - refactor, rename MatchingGraph, impl Graph for MatchingGraph

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Matching {
    pub edges: Vec<UndirectedEdge>,
}

impl Matching {
    pub fn new() -> Self {
        Matching { edges: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct MatchingGraph {
    vertices: Vec<MatchingVertex>,
    size: usize,
}

#[derive(Debug, Clone)]
pub struct MatchingVertex {
    index: usize,
    active: bool,
    neighbors: Vec<usize>,
}

impl MatchingVertex {
    pub fn new(index: usize) -> Self {
        MatchingVertex {
            index,
            active: true,
            neighbors: vec![],
        }
    }

    pub fn new_non_active(index: usize) -> Self {
        MatchingVertex {
            index,
            active: false,
            neighbors: vec![],
        }
    }

    pub fn index(&self) -> &usize {
        &self.index
    }

    #[allow(dead_code)]
    pub fn neighbors(&self) -> &Vec<usize> {
        &self.neighbors
    }
}

#[allow(dead_code)]
impl MatchingGraph {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        MatchingGraph {
            vertices: Vec::with_capacity(capacity),
            size: 0,
        }
    }

    pub fn from_graph<G: Graph>(graph: &G) -> Self {
        let mut match_graph = MatchingGraph::with_capacity(graph.size());
        for edge in graph.edges() {
            match_graph.add_edge(edge.from(), edge.to());
        }
        match_graph
    }

    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        if !self.vertices[from].active || !self.vertices[to].active {
            return false;
        }
        let mut first = false;
        for neighbor in self.neighbors(from) {
            if neighbor == &to {
                first = true;
                break;
            }
        }
        if !first {
            return false;
        }
        let mut second = false;
        for neighbor in self.neighbors(to) {
            if neighbor == &from {
                second = true;
            }
        }
        if !second {
            return false;
        }
        true
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.create_vertex_if_not_exists(from);
        self.create_vertex_if_not_exists(to);
        self.vertices[from]
            .neighbors
            .retain(|neighbor| neighbor != &to);
        self.vertices[from].neighbors.push(to);
        self.vertices[to]
            .neighbors
            .retain(|neighbor| neighbor != &from);
        self.vertices[to].neighbors.push(from);
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) {
        self.vertices[from]
            .neighbors
            .retain(|neighbor| neighbor != &to);
        self.vertices[to]
            .neighbors
            .retain(|neighbor| neighbor != &from);
    }

    ///
    /// if given vertex is removed but has neighbors - will be recovered as is
    ///
    pub fn create_vertex_if_not_exists(&mut self, vertex: usize) {
        if self.vertices.len() > vertex {
            if !self.vertices[vertex].active {
                self.size += 1;
                self.vertices[vertex].active = true;
                self.vertices[vertex].neighbors = vec![];
            }
            return;
        }
        while self.vertices.len() <= vertex {
            if self.vertices.len() == vertex {
                self.vertices.push(MatchingVertex::new(self.vertices.len()));
                self.size += 1;
            } else {
                self.vertices
                    .push(MatchingVertex::new_non_active(self.vertices.len()));
            }
        }
    }

    pub fn add_vertex(&mut self, vertex: MatchingVertex) {
        self.create_vertex_if_not_exists(vertex.index);
        for neighbor in vertex.neighbors.iter() {
            self.create_vertex_if_not_exists(*neighbor);
            let mut already_has = false;
            for neighbor in self.vertices[*neighbor].neighbors.iter() {
                if neighbor == &vertex.index {
                    already_has = true;
                    break;
                }
            }
            if !already_has {
                self.vertices[*neighbor].neighbors.push(vertex.index);
            }
        }
        self.vertices[vertex.index].neighbors = vertex.neighbors;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn max_vertex_index(&self) -> usize {
        self.vertices.len()
    }

    pub fn vertices(&self) -> MatchingGraphVerticesIter {
        MatchingGraphVerticesIter {
            vertices: self.vertices.iter(),
        }
    }

    ///
    /// can crash if vertex >= self.vertices.len()
    ///
    pub fn neighbors(&self, vertex: usize) -> &Vec<usize> {
        &self.vertices[vertex].neighbors
    }

    fn first_vertex(&self) -> Option<&MatchingVertex> {
        for vertex in self.vertices.iter() {
            if vertex.active {
                return Some(vertex);
            }
        }
        None
    }

    pub fn remove_vertex(&mut self, vertex: usize) {
        let neighbors = self.vertices[vertex].neighbors.clone();
        for neighbor in neighbors {
            self.vertices[neighbor]
                .neighbors
                .retain(|neighb| neighb != &vertex);
        }
        self.vertices[vertex].active = false;
        self.size -= 1;
    }

    pub fn has_odd_size_component(&mut self) -> bool {
        let mut removed_vertices = vec![];
        while let Some(start) = self.first_vertex() {
            let mut bfs_graph = BfsGraph::new(self, start.index);
            let mut visited_vertices = vec![];
            while let Some(vertex) = bfs_graph.bfs_next() {
                visited_vertices.push(vertex);
            }
            if visited_vertices.len() % 2 == 1 {
                self.activate_vertices(removed_vertices);
                return true;
            }

            // shortcut
            if visited_vertices.len() == self.size() {
                return false;
            }
            // remove visited vertices
            for visited_vertex in visited_vertices {
                removed_vertices.push(visited_vertex);
                self.vertices[visited_vertex].active = false;
            }
        }
        self.activate_vertices(removed_vertices);
        false
    }

    fn activate_vertices(&mut self, vertices_to_activate: Vec<usize>) {
        for vertex_to_activate in vertices_to_activate {
            self.vertices[vertex_to_activate].active = true;
        }
    }

    pub fn perfect_matchings(&mut self) -> Vec<Matching> {
        let mut matchings = vec![];
        if self.size == 0 {
            matchings.push(Matching::new());
            return matchings;
        }
        if !self.has_odd_size_component() {
            if let Some(vertex) = self.first_vertex() {
                let vertex = vertex.clone();
                for neighbor in vertex.neighbors.iter() {
                    // remove vertices
                    let mut removed_vertices = vec![];
                    removed_vertices.push(self.vertices[vertex.index].clone());
                    removed_vertices.push(self.vertices[*neighbor].clone());

                    self.remove_vertex(vertex.index);
                    self.remove_vertex(*neighbor);
                    let mut matchings_local = self.perfect_matchings();
                    for matching in matchings_local.iter_mut() {
                        matching
                            .edges
                            .push(UndirectedEdge::new(vertex.index, *neighbor));
                    }
                    matchings.append(&mut matchings_local);
                    // recover removed vertices
                    for removed_vertex in removed_vertices {
                        self.add_vertex(removed_vertex);
                    }
                }
            }
        }
        matchings
    }
}

pub struct MatchingGraphVerticesIter<'a> {
    vertices: slice::Iter<'a, MatchingVertex>,
}

impl<'a> Iterator for MatchingGraphVerticesIter<'a> {
    type Item = &'a MatchingVertex;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(vertex) = self.vertices.next() {
            if vertex.active {
                return Some(vertex);
            }
        }
        None
    }
}

pub struct BfsGraph<'a> {
    graph: &'a MatchingGraph,
    visited: Vec<bool>,
    to_visit: VecDeque<usize>,
}

impl<'a> BfsGraph<'a> {
    pub fn new(graph: &'a MatchingGraph, start: usize) -> Self {
        let visited = vec![false; graph.vertices.len()];
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
            for neighbor in self.graph.neighbors(vertex) {
                if self.visit(*neighbor) {
                    self.to_visit.push_back(*neighbor);
                }
            }
            return Some(vertex);
        }
        None
    }
}
