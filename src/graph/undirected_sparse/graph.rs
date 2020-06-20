use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::graph::{Graph, GraphConstructor};
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::vertex::SimpleVertex;
use crate::graph::undirected_sparse::vertex::VertexWithEdges;
use crate::graph::vertex::{Vertex, VertexConstructor};
use std::slice::{Iter, IterMut};
use std::{fmt, marker, slice};

/// best for sparse graphs
#[derive(Debug, Clone)]
pub struct SimpleSparseGraph {
    pub size: usize,
    pub vertices: Vec<VertexWithEdges>,
}

impl Graph for SimpleSparseGraph {
    type V = VertexWithEdges;
    type E = UndirectedEdge;

    fn size(&self) -> usize {
        self.vertices.len()
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        if from >= self.vertices.len() || to >= self.vertices.len() {
            return false;
        }
        let from_vertex = &self.vertices[from];
        for edge in from_vertex.edges.iter() {
            if edge.from() == from && edge.to() == to || edge.from() == to && edge.to() == from {
                return true;
            }
        }
        false
    }

    fn add_vertex(&mut self) {
        self.vertices
            .push(VertexWithEdges::new(self.vertices.len()));
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        let edge = UndirectedEdge::new(from, to);
        if self.has_edge(from, to) {
            return;
        }
        while self.vertices.len() <= edge.to() {
            self.add_vertex();
        }
        let from_vertex = &mut self.vertices[from];
        from_vertex.add_edge(to, 0);
        let to_vertex = &mut self.vertices[to];
        to_vertex.add_edge(from, 0);
    }

    fn remove_edge(&mut self, from: usize, to: usize) {
        let edge_to_remove = UndirectedEdge::new(from, to);

        let from_vertex = &mut self.vertices[from];
        from_vertex.edges.retain(|edge| {
            edge.from() != edge_to_remove.from() || edge.to() != edge_to_remove.to()
        });

        let to_vertex = &mut self.vertices[to];
        to_vertex.edges.retain(|edge| {
            edge.from() != edge_to_remove.from() || edge.to() != edge_to_remove.to()
        });
    }

    fn remove_edges_of_vertex(&mut self, vertex: usize) {
        let neighbors = self.vertices[vertex].neighbors().clone();
        self.vertices[vertex].edges = vec![];
        for neighbor in neighbors {
            let edge_to_remove = UndirectedEdge::new(vertex, neighbor);
            self.vertices[neighbor].edges.retain(|edge| {
                edge_to_remove.from() != edge.from() || edge_to_remove.to() != edge.to()
            });
        }
    }

    fn vertices<'a>(&'a self) -> Box<dyn Iterator<Item = &'a VertexWithEdges> + 'a> {
        Box::new(self.vertices.iter())
    }

    fn edges<'a>(&'a self) -> Box<dyn Iterator<Item = &'a UndirectedEdge> + 'a> {
        Box::new(Edges::new(&self.vertices))
    }

    fn edges_of_vertex<'a>(
        &'a self,
        vertex: usize,
    ) -> Box<dyn Iterator<Item = &'a UndirectedEdge> + 'a> {
        Box::new(self.vertices[vertex].edges.iter())
    }
}

impl GraphConstructor for SimpleSparseGraph {
    fn new() -> Self {
        Self::with_vertices_capacity(20)
    }

    fn with_capacity(vertices: usize, edges: usize) -> Self {
        Self::with_vertices_capacity(vertices)
    }

    fn with_vertices_capacity(vertices: usize) -> Self {
        SimpleSparseGraph {
            size: 0,
            vertices: Vec::with_capacity(vertices),
        }
    }
}

impl SimpleSparseGraph {
    pub fn from_graph<G: Graph>(graph: &G) -> Self {
        let mut result = SimpleSparseGraph::with_vertices_capacity(graph.size());
        result.size = graph.size();
        for edge in graph.edges() {
            result.add_edge(edge.from(), edge.to());
        }
        result
    }
}

/// Edges
pub struct Edges<'a> {
    vertices: &'a Vec<VertexWithEdges>,
    position: (usize, usize),
}

impl<'a> Edges<'a> {
    pub fn new(vertices: &'a Vec<VertexWithEdges>) -> Self {
        Edges {
            vertices,
            position: (0, 0),
        }
    }
}

impl<'a> Iterator for Edges<'a> {
    type Item = &'a UndirectedEdge;
    fn next(&mut self) -> Option<Self::Item> {
        if self.vertices.len() <= self.position.0 {
            return None;
        }

        let vertex = &self.vertices[self.position.0];
        if vertex.edges.len() <= self.position.1 {
            self.position.0 += 1;
            self.position.1 = 0;
            return self.next();
        }

        let neighboring_edge = &vertex.edges[self.position.1];

        if neighboring_edge.from() != vertex.index() {
            self.position.1 += 1;
            return self.next();
        }

        self.position.1 += 1;
        Some(neighboring_edge)
    }
}

impl fmt::Display for SimpleSparseGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for vertex in &self.vertices {
            write!(f, "{}: ", vertex.index())?;
            let mut separ = String::from("");
            for edge in vertex.edges.iter() {
                let neighbor = if edge.from() == vertex.index() {
                    edge.to()
                } else {
                    edge.from()
                };
                write!(f, "{}{}", separ, neighbor)?;
                separ = String::from(", ");
            }

            writeln!(f)?;
        }
        Ok(())
    }
}

// TODO - to compare like this - we need to sort edges after new edge to vertex is added
impl PartialEq for SimpleSparseGraph {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        if self.vertices[..] != other.vertices[..] {
            return false;
        }
        true
    }
}
