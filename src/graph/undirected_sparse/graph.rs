use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected_sparse::vertex::{Neighbor, VertexWithNeighbors};
use std::slice::{Iter, IterMut};
use std::{fmt, marker, slice};

// best for sparse graphs

// TODO - adjust Graph trait (edges() -> Edges), then implement

#[derive(Debug, Clone)]
pub struct SimpleSparseGraph {
    pub size: usize,
    pub vertices: Vec<VertexWithNeighbors>,
}

impl SimpleSparseGraph {
    pub fn size(&self) -> usize {
        self.vertices.len()
    }

    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        if from >= self.vertices.len() || to >= self.vertices.len() {
            return false;
        }
        let from_vertex = &self.vertices[from];
        for neighbor in from_vertex.neighbors.iter() {
            if neighbor.index() == to {
                return true;
            }
        }
        false
    }

    pub fn add_vertex(&mut self) {
        self.vertices
            .push(VertexWithNeighbors::new(self.vertices.len()));
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        let edge = UndirectedEdge::new(from, to);
        if self.has_edge(from, to) {
            return;
        }
        while self.vertices.len() <= edge.to() {
            self.add_vertex();
        }
        let from_vertex = &mut self.vertices[from];
        from_vertex.add_neighbor(to, 0);
        let to_vertex = &mut self.vertices[to];
        to_vertex.add_neighbor(from, 0);
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) {
        let from_vertex = &mut self.vertices[from];
        from_vertex
            .neighbors
            .retain(|neighbor| neighbor.index() != to);

        let to_vertex = &mut self.vertices[to];
        to_vertex
            .neighbors
            .retain(|neighbor| neighbor.index() != from);
    }

    pub fn remove_edges_of_vertex(&mut self, vertex: usize) {
        let neighbors = self.vertices[vertex].neighbors.clone();
        self.vertices[vertex].neighbors = vec![];

        for neighbor in neighbors {
            self.vertices[neighbor.index()]
                .neighbors
                .retain(|neighbor| neighbor.index() != vertex);
        }
    }

    pub fn vertices(&self) -> Iter<VertexWithNeighbors> {
        self.vertices.iter()
    }

    pub fn vertices_mut(&mut self) -> IterMut<VertexWithNeighbors> {
        self.vertices.iter_mut()
    }

    pub fn edges(&self) -> Edges<UndirectedEdge> {
        Edges::new(&self.vertices)
    }

    pub fn with_capacity(vertices: usize) -> Self {
        SimpleSparseGraph {
            size: 0,
            vertices: Vec::with_capacity(vertices),
        }
    }

    pub fn from_graph<G: Graph>(graph: &G) -> Self {
        let mut result = SimpleSparseGraph::with_capacity(graph.size());
        for edge in graph.edges() {
            result.add_edge(edge.from(), edge.to());
        }
        result
    }
}

/// Edges
pub struct Edges<'a, E> {
    vertices: &'a Vec<VertexWithNeighbors>,
    position: (usize, usize),

    _ph: marker::PhantomData<E>,
}

impl<'a, E> Edges<'a, E> {
    pub fn new(vertices: &'a Vec<VertexWithNeighbors>) -> Self {
        Edges {
            vertices,
            position: (0, 0),
            _ph: marker::PhantomData,
        }
    }
}

impl<'a, E> Iterator for Edges<'a, E>
where
    E: Edge + EdgeConstructor,
{
    type Item = E;
    fn next(&mut self) -> Option<Self::Item> {
        if self.vertices.len() <= self.position.0 {
            return None;
        }

        let vertex = &self.vertices[self.position.0];
        if vertex.neighbors.len() <= self.position.1 {
            self.position.0 += 1;
            self.position.1 = 0;
            return self.next();
        }

        let neighbor = &vertex.neighbors[self.position.1];
        if neighbor.index() < vertex.index() {
            self.position.1 += 1;
            return self.next();
        }

        self.position.1 += 1;
        Some(E::new_with_colour(
            vertex.index(),
            neighbor.index(),
            neighbor.colour(),
        ))
    }
}

impl fmt::Display for SimpleSparseGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for vertex in &self.vertices {
            write!(f, "{}: ", vertex.index())?;
            let mut separ = String::from("");
            for neighbor in vertex.neighbors.iter() {
                write!(f, "{}{}", separ, neighbor.index())?;
                separ = String::from(", ");
            }

            writeln!(f)?;
        }
        Ok(())
    }
}
