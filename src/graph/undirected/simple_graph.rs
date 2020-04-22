use std::fmt;

use crate::graph::edge::Edge;
use crate::graph::graph::{Edges, EdgesMut, Graph, Vertices, VerticesMut};
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::vertex::SimpleVertex;
use crate::graph::vertex::Vertex;

#[derive(Debug)]
pub struct SimpleGraph {
    size: usize,
    vertices: Vec<SimpleVertex>,
    edges: Vec<UndirectedEdge>,
    // impl hash map for vert->edges ?? - for fast edge retrieval (edges of vertex)
}

/// undirected, without loop, without multiple edges
impl Graph for SimpleGraph {
    fn size(&self) -> usize {
        self.vertices.len()
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        let edge_to_check = UndirectedEdge::new(from, to);
        for edge in &self.edges {
            if edge.from() == edge_to_check.from() && edge.to() == edge_to_check.to() {
                return true;
            }
        }
        false
    }

    fn edge(&self, from: usize, to: usize) -> Option<UndirectedEdge> {
        println!("{} {}", from, to);
        unimplemented!()
    }

    fn add_vertex(&mut self) {
        self.vertices.push(SimpleVertex::new(self.vertices.len()));
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        let edge = UndirectedEdge::new(from, to);
        if self.has_edge(from, to) {
            return;
        }
        self.edges.push(edge.clone());
        self.edges.sort_by(|a, b| {
            if a.from() == b.from() {
                return a.to().cmp(&b.to());
            }
            a.from().cmp(&b.from())
        });
        while self.vertices.len() <= edge.to() {
            self.add_vertex();
        }
    }

    fn vertices(&self) -> Vertices<SimpleVertex> {
        Vertices::new(self.vertices.iter())
    }

    fn vertices_mut(&mut self) -> VerticesMut<SimpleVertex> {
        VerticesMut::new(self.vertices.iter_mut())
    }

    fn edges(&self) -> Edges<UndirectedEdge> {
        Edges::new(self.edges.iter())
    }

    fn edges_mut(&mut self) -> EdgesMut<UndirectedEdge> {
        EdgesMut::new(self.edges.iter_mut())
    }

    fn edges_of_vertex(&self, vertex: usize) -> Edges<UndirectedEdge> {
        Edges::of_vertex(self.edges.iter(), vertex)
    }

    fn with_capacity(vertices: usize, edges: usize) -> Self {
        SimpleGraph {
            size: 0,
            vertices: Vec::with_capacity(vertices),
            edges: Vec::with_capacity(edges),
        }
    }
}

impl fmt::Display for SimpleGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for vertex in &self.vertices {
            write!(f, "{}: ", vertex.index())?;
            let mut separ = String::from("");
            for edges_of_vertex in self.edges_of_vertex(vertex.index()) {
                if edges_of_vertex.from() == vertex.index() {
                    write!(f, "{}{}", separ, edges_of_vertex.to())?;
                } else {
                    write!(f, "{}{}", separ, edges_of_vertex.from())?;
                }
                separ = String::from(", ");
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl PartialEq for SimpleGraph {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        if self.edges[..] != other.edges[..] {
            return false;
        }
        if self.vertices[..] != other.vertices[..] {
            return false;
        }
        true
    }
}

impl Eq for SimpleGraph {}
