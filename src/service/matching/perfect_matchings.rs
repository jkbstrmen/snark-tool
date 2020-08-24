use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use std::collections::hash_map;
use std::collections::{HashMap, VecDeque};
use std::{iter, slice};

// TODO - rename MatchingGraph, impl Graph for MatchingGraph

#[derive(Debug, Clone)]
pub struct MatchingGraph {
    vertices: HashMap<usize, Vec<usize>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Matching {
    pub edges: Vec<UndirectedEdge>,
}

impl Matching {
    pub fn new() -> Self {
        Matching { edges: vec![] }
    }

    // necessary?
    // pub fn sort_edges(&mut self) {
    //     self.edges.sort_by(|a, b| Self::edge_compare(a, b));
    // }
    //
    // fn edge_compare(first: &UndirectedEdge, second: &UndirectedEdge) -> cmp::Ordering {
    //     let compare_from = first.from().cmp(&second.from());
    //     if cmp::Ordering::Equal.eq(&compare_from) {
    //         return first.to().cmp(&second.to());
    //     }
    //     compare_from
    // }
    //
    // pub fn vertices(&self) -> Vec<usize> {
    //     let mut vertices = vec![];
    //     for edge in self.edges.iter() {
    //         vertices.push(edge.from());
    //         vertices.push(edge.to());
    //     }
    //     vertices
    // }
}

impl MatchingGraph {
    pub fn new() -> Self {
        MatchingGraph {
            vertices: HashMap::new(),
        }
    }

    pub fn from_graph<G: Graph>(graph: &G) -> Self {
        let mut match_graph = MatchingGraph::new();
        for edge in graph.edges() {
            match_graph.add_edge(edge.from(), edge.to());
        }
        match_graph
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.add_neighbor(from, to);
        self.add_neighbor(to, from);
    }

    pub fn add_vertex(&mut self, vertex: usize) {
        if let None = self.vertices.get_mut(&vertex) {
            self.vertices.insert(vertex, vec![]);
        }
    }

    fn add_neighbor(&mut self, vertex: usize, neighbor: usize) {
        if let Some(neighbors) = self.vertices.get_mut(&vertex) {
            neighbors.push(neighbor);
        } else {
            self.vertices.insert(vertex, vec![neighbor]);
        }
    }

    pub fn size(&self) -> usize {
        // self.vertices.len()
        let mut max = 0;
        for key in self.vertices.keys() {
            if *key > max {
                max = *key
            }
        }
        max + 1
    }

    pub fn vertices(&self) -> hash_map::Iter<usize, Vec<usize>> {
        self.vertices.iter()
    }

    pub fn neighbors(&self, vertex: usize) -> Option<&Vec<usize>> {
        self.vertices.get(&vertex)

        // if neighbors.is_some() {
        //     return neighbors.unwrap().iter();
        // }
        // unimplemented!()
    }

    fn first_vertex(&self) -> Option<(&usize, &Vec<usize>)> {
        for vertex in self.vertices.iter() {
            return Some(vertex);
        }
        None
    }

    pub fn remove_vertex(&mut self, vertex: usize) -> Option<Vec<usize>> {
        if let Some(neighbors) = self.vertices.get(&vertex) {
            let copy = neighbors.clone();
            for neighbor in copy {
                self.remove_neighbor(neighbor, vertex);
            }
        }
        self.vertices.remove(&vertex)
    }

    fn remove_neighbor(&mut self, vertex: usize, neighbor_to_remove: usize) {
        if let Some(neighbors) = self.vertices.get_mut(&vertex) {
            neighbors.retain(|neighbor| &neighbor_to_remove != neighbor);
        }
    }

    pub fn has_odd_size_component(&self) -> bool {
        let mut graph_copy = self.clone();
        while let Some(start) = graph_copy.first_vertex() {
            let mut bfs_graph = BfsGraph::new(&graph_copy, *start.0);
            let mut visited_vertices = vec![];
            while let Some(vertex) = bfs_graph.bfs_next() {
                visited_vertices.push(vertex);
            }
            if visited_vertices.len() % 2 == 1 {
                return true;
            }
            // remove visited vertices from graph_copy
            for visited_vertex in visited_vertices {
                graph_copy.vertices.remove(&visited_vertex);
            }
        }
        false
    }

    pub fn perfect_matchings(&self) -> Vec<Matching> {
        if self.vertices.is_empty() {
            let mut matchings = vec![];
            matchings.push(Matching::new());
            return matchings;
        }

        let mut matchings = vec![];
        if !self.has_odd_size_component() {
            let vertex = self.first_vertex().unwrap();

            for neighbor in vertex.1 {
                let mut graph_copy = self.clone();
                graph_copy.remove_vertex(*vertex.0);
                graph_copy.remove_vertex(*neighbor);

                let mut matchings_local = graph_copy.perfect_matchings();
                for mut matching in matchings_local.iter_mut() {
                    matching
                        .edges
                        .push(UndirectedEdge::new(*vertex.0, *neighbor));
                    matching.edges.sort();
                }
                matchings.append(&mut matchings_local);
            }
        }
        matchings
    }
}

pub struct BfsGraph<'a> {
    graph: &'a MatchingGraph,

    // using Vec<bool> to indicate visited vertex seems to have  better performance
    visited: Vec<bool>,
    // visited: HashMap<usize, bool>,

    to_visit: VecDeque<usize>,
}

impl<'a> BfsGraph<'a> {
    pub fn new(graph: &'a MatchingGraph, start: usize) -> Self {
        // let mut visited: HashMap<usize, bool> = HashMap::new();
        let mut visited = vec![false; graph.size()];
        let mut to_visit = VecDeque::new();
        to_visit.push_back(start);

        let mut bfs = BfsGraph {
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

        // let old_val = self.visited.insert(vertex, true);
        // if old_val.is_some() {
        //     return false;
        // }
        // true
    }

    pub fn bfs_next(&mut self) -> Option<usize> {
        if let Some(vertex) = self.to_visit.pop_front() {
            if let Some(neighbors) = self.graph.neighbors(vertex) {
                for neighbor in neighbors {
                    if self.visit(*neighbor) {
                        self.to_visit.push_back(*neighbor);
                    }
                }
            }
            return Some(vertex);
        }
        None
    }
}
