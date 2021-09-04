use std::{fmt, slice};

use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::graph::{Graph, GraphConstructor};
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::simple_edge_graph::simple_vertex::SimpleVertex;
use crate::graph::undirected::UndirectedGraph;
use crate::graph::vertex::{Vertex, VertexConstructor};
use serde::export::Option::Some;

///
/// undirected, without loops or multiple edges
///
#[derive(Debug, Clone)]
pub struct SimpleEdgeGraph {
    pub size: usize,
    pub vertices: Vec<SimpleVertex>,
    pub edges: Vec<UndirectedEdge>,
}

impl UndirectedGraph for SimpleEdgeGraph {}

impl Graph for SimpleEdgeGraph {
    type V = SimpleVertex;
    type E = UndirectedEdge;

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

    fn remove_edge(&mut self, from: usize, to: usize) {
        let to_remove = UndirectedEdge::new(from, to);
        self.edges
            .retain(|edge| edge.from() != to_remove.from() || edge.to() != to_remove.to());
    }

    fn remove_edges_of_vertex(&mut self, vertex: usize) /*-> Self*/
    {
        self.edges
            .retain(|edge| edge.from() != vertex && edge.to() != vertex);
    }

    fn remove_vertex(&mut self, vertex_index: usize) {
        self.remove_edges_of_vertex(vertex_index)
    }

    fn vertices<'a>(&'a self) -> Box<dyn Iterator<Item = &'a SimpleVertex> + 'a> {
        let iter: slice::Iter<'a, SimpleVertex> = self.vertices.iter();
        Box::new(iter)
    }

    fn edges<'a>(&'a self) -> Box<dyn Iterator<Item = &'a UndirectedEdge> + 'a> {
        Box::new(self.edges.iter())
    }

    fn edges_of_vertex<'a>(
        &'a self,
        vertex: usize,
    ) -> Box<dyn Iterator<Item = &'a UndirectedEdge> + 'a> {
        Box::new(Edges::of_vertex(self.edges.iter(), vertex))
    }

    fn neighbors_of_vertex(&self, vertex: usize) -> Vec<usize> {
        let mut neighbors = vec![];
        let mut edges = self.edges_of_vertex(vertex);
        while let Some(edge) = edges.next() {
            if edge.from() == vertex {
                neighbors.push(edge.to());
            } else {
                neighbors.push(edge.from());
            }
        }
        neighbors
    }
}

impl GraphConstructor for SimpleEdgeGraph {
    fn new() -> Self {
        Self::with_vertices_capacity(20)
    }

    fn with_capacity(vertices: usize, edges: usize) -> Self {
        SimpleEdgeGraph {
            size: 0,
            vertices: Vec::with_capacity(vertices),
            edges: Vec::with_capacity(edges),
        }
    }

    fn with_vertices_capacity(vertices: usize) -> Self {
        Self::with_capacity(vertices, vertices * 2)
    }
}

impl SimpleEdgeGraph {
    #[allow(dead_code)]
    pub fn from_graph<G: Graph>(graph: &G) -> Self {
        let mut vertices = vec![];
        for vertex in graph.vertices() {
            vertices.push(SimpleVertex::new(vertex.index()));
        }
        let mut edges = vec![];
        for edge in graph.edges() {
            edges.push(UndirectedEdge::new(edge.from(), edge.to()));
        }
        SimpleEdgeGraph {
            size: graph.size(),
            vertices,
            edges,
        }
    }
}

impl fmt::Display for SimpleEdgeGraph {
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

impl PartialEq for SimpleEdgeGraph {
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

impl Eq for SimpleEdgeGraph {}

/// Edges
pub struct Edges<'a, E> {
    vertex: Option<usize>,
    iter: slice::Iter<'a, E>,
}

impl<'a, E> Edges<'a, E> {
    pub fn of_vertex(iter: slice::Iter<'a, E>, vertex: usize) -> Self {
        Edges {
            vertex: Some(vertex),
            iter,
        }
    }
}

impl<'a, E> Iterator for Edges<'a, E>
where
    E: Edge,
{
    type Item = &'a E;
    fn next(&mut self) -> Option<Self::Item> {
        if self.vertex.is_some() {
            let mut edge_opt = self.iter.next();
            while edge_opt.is_some() {
                let edge = edge_opt.unwrap();
                if edge.from() == self.vertex.unwrap() || edge.to() == self.vertex.unwrap() {
                    return edge_opt;
                }
                edge_opt = self.iter.next();
            }
            return None;
        }
        self.iter.next()
    }
}
