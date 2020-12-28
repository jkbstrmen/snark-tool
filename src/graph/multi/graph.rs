use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::graph::{Graph, GraphConstructor};
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected_sparse::vertex::VertexWithEdges;
use crate::graph::vertex::{Vertex, VertexConstructor};
use std::{fmt, slice};

///
/// Can hold multiple edges from one vertex to another
///
#[derive(Debug, Clone)]
pub struct MultiGraph {
    pub vertices: Vec<VertexWithEdges>,
}

impl Graph for MultiGraph {
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
        if from == to {
            return;
        }
        while self.vertices.len() <= edge.to() {
            self.add_non_active_vertex();
        }
        let from_vertex = &mut self.vertices[from];
        from_vertex.add_edge(to, 0);
        from_vertex.set_active(true);
        let to_vertex = &mut self.vertices[to];
        to_vertex.add_edge(from, 0);
        to_vertex.set_active(true);
    }

    fn remove_edge(&mut self, from: usize, to: usize) {
        let edge_to_remove = UndirectedEdge::new(from, to);

        let from_vertex = &mut self.vertices[from];

        let pos = from_vertex.edges.iter().position(|x| *x == edge_to_remove);
        if let Some(position) = pos {
            from_vertex.edges.remove(position);
        }

        // from_vertex.edges.remove_item(&edge_to_remove);
        //
        // from_vertex.edges.retain(|edge| {
        //     edge.from() != edge_to_remove.from() || edge.to() != edge_to_remove.to()
        // });

        let to_vertex = &mut self.vertices[to];
        let pos = to_vertex.edges.iter().position(|x| *x == edge_to_remove);
        if let Some(position) = pos {
            to_vertex.edges.remove(position);
        }
        // to_vertex.edges.retain(|edge| {
        //     edge.from() != edge_to_remove.from() || edge.to() != edge_to_remove.to()
        // });
    }

    fn remove_edges_of_vertex(&mut self, vertex: usize) {
        if vertex >= self.vertices.len() {
            return;
        }
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
        Box::new(Vertices::new(self.vertices.iter()))
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
        neighbors.sort();
        neighbors.dedup();
        neighbors
    }
}

impl GraphConstructor for MultiGraph {
    fn new() -> Self {
        Self::with_vertices_capacity(20)
    }

    fn with_capacity(vertices: usize, _edges: usize) -> Self {
        Self::with_vertices_capacity(vertices)
    }

    fn with_vertices_capacity(vertices: usize) -> Self {
        MultiGraph {
            vertices: Vec::with_capacity(vertices),
        }
    }
}

impl MultiGraph {
    pub fn from_graph<G: Graph>(graph: &G) -> Self {
        let mut result = MultiGraph::with_vertices_capacity(graph.size());
        for edge in graph.edges() {
            result.add_edge(edge.from(), edge.to());
        }
        result
    }

    ///
    /// for now has_vertex for vertex v is true if graph contains edge connected to vertex v
    ///
    pub fn has_vertex(&self, vertex: usize) -> bool {
        if vertex >= self.size() {
            return false;
        }
        self.vertices[vertex].active()
    }

    pub fn first_vertex(&self) -> Option<&VertexWithEdges> {
        for vertex in self.vertices() {
            if self.has_vertex(vertex.index()) {
                return Some(vertex);
            }
        }
        None
    }

    pub fn vertex(&self, index: usize) -> Option<&VertexWithEdges> {
        if !self.has_vertex(index) {
            return None;
        }
        Some(&self.vertices[index])
    }

    pub fn remove_vertex(&mut self, vertex: usize) {
        if vertex >= self.vertices.len() {
            return;
        }
        self.remove_edges_of_vertex(vertex);
        self.vertices[vertex].set_active(false);
    }

    fn add_non_active_vertex(&mut self) {
        self.vertices
            .push(VertexWithEdges::new_non_active(self.vertices.len()));
    }
}

/// Vertices
pub struct Vertices<'a> {
    vertices: slice::Iter<'a, VertexWithEdges>,
}

impl<'a> Vertices<'a> {
    pub fn new(vertices: slice::Iter<'a, VertexWithEdges>) -> Self {
        Vertices {
            vertices,
        }
    }
}

impl<'a> Iterator for Vertices<'a> {
    type Item = &'a VertexWithEdges;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(vertex) = self.vertices.next() {
            if vertex.active() {
                return Some(vertex);
            }
            return self.next();
        }
        None
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

impl fmt::Display for MultiGraph {
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
impl PartialEq for MultiGraph {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!();

        if self.size() != other.size() {
            return false;
        }
        if self.vertices[..] != other.vertices[..] {
            return false;
        }
        true
    }
}
