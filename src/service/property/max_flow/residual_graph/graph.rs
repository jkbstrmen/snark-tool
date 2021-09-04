use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::graph::{Graph, GraphConstructor};
use crate::graph::undirected::multi_graph::graph::MultiGraph;
use crate::graph::vertex::{Vertex, VertexConstructor};
use crate::service::property::max_flow::residual_graph::edge::DirectedFlowEdge;
use crate::service::property::max_flow::residual_graph::vertex::ResidualVertex;
use serde::export::Option::Some;
use std::{fmt, slice};

///
/// better for sparse graphs
/// use VertexWithEdges - faster edge addition and removal
///
#[derive(Debug, Clone)]
pub struct ResidualGraph {
    pub vertices: Vec<ResidualVertex>,
}

impl Graph for ResidualGraph {
    type V = ResidualVertex;
    type E = DirectedFlowEdge;

    fn size(&self) -> usize {
        self.vertices.len()
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        if from >= self.vertices.len() || to >= self.vertices.len() {
            return false;
        }
        let from_vertex = &self.vertices[from];
        from_vertex.has_neighbor(to)
    }

    fn add_vertex(&mut self) {
        self.vertices.push(ResidualVertex::new(self.vertices.len()));
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        let edge = DirectedFlowEdge::new(from, to);
        if from == to {
            return;
        }
        if self.has_edge(from, to) {
            self.increase_edge_capacity(from, to);
            return;
        }
        while self.vertices.len() <= edge.to() || self.vertices.len() <= edge.from() {
            self.add_non_active_vertex();
        }
        let from_vertex = &mut self.vertices[from];
        from_vertex.add_edge(to);
        from_vertex.set_active(true);
    }

    fn remove_edge(&mut self, from: usize, to: usize) {
        let edge_to_remove = DirectedFlowEdge::new(from, to);

        let from_vertex = &mut self.vertices[from];
        from_vertex.edges.retain(|edge| {
            edge.from() != edge_to_remove.from() || edge.to() != edge_to_remove.to()
        });
    }

    fn remove_edges_of_vertex(&mut self, vertex: usize) {
        if vertex >= self.vertices.len() {
            return;
        }
        self.vertices[vertex].edges = vec![];
    }

    fn remove_vertex(&mut self, vertex_index: usize) {
        self.remove_edges_of_vertex(vertex_index)
    }

    fn vertices<'a>(&'a self) -> Box<dyn Iterator<Item = &'a ResidualVertex> + 'a> {
        Box::new(Vertices::new(self.vertices.iter()))
    }

    fn edges<'a>(&'a self) -> Box<dyn Iterator<Item = &'a DirectedFlowEdge> + 'a> {
        Box::new(Edges::new(&self.vertices))
    }

    fn edges_of_vertex<'a>(
        &'a self,
        vertex: usize,
    ) -> Box<dyn Iterator<Item = &'a DirectedFlowEdge> + 'a> {
        Box::new(self.vertices[vertex].edges.iter())
    }

    fn neighbors_of_vertex(&self, vertex: usize) -> Vec<usize> {
        if let Some(vertex) = self.vertex(vertex) {
            return vertex.neighbors();
        }
        vec![]
    }
}

impl GraphConstructor for ResidualGraph {
    fn new() -> Self {
        Self::with_vertices_capacity(20)
    }

    fn with_capacity(vertices: usize, _edges: usize) -> Self {
        Self::with_vertices_capacity(vertices)
    }

    fn with_vertices_capacity(vertices: usize) -> Self {
        ResidualGraph {
            vertices: Vec::with_capacity(vertices),
        }
    }
}

impl ResidualGraph {
    // ///
    // /// only for graphs without edge capacity
    // /// every edge is taken as with capacity of 1
    // ///
    // pub fn from_graph<G: Graph>(graph: &G) -> Self {
    //     let mut result = ResidualGraph::with_vertices_capacity(graph.size());
    //     for edge in graph.edges() {
    //         result.add_edge(edge.from(), edge.to());
    //     }
    //     result
    // }

    pub fn from_multi_graph(graph: &MultiGraph) -> Self {
        let mut result = ResidualGraph::with_vertices_capacity(graph.size());
        for edge in graph.edges() {
            result.add_edge(edge.from(), edge.to());
            result.add_edge(edge.to(), edge.from());
        }
        result
    }

    #[allow(dead_code)]
    pub fn add_vertex_with_index(&mut self, vertex: usize) {
        while self.size() < vertex + 1 {
            self.add_non_active_vertex();
        }
        self.vertices[vertex].set_active(true);
    }

    pub fn has_vertex(&self, vertex: usize) -> bool {
        if vertex >= self.size() {
            return false;
        }
        self.vertices[vertex].active()
    }

    #[allow(dead_code)]
    pub fn first_vertex(&self) -> Option<&ResidualVertex> {
        for vertex in self.vertices() {
            if self.has_vertex(vertex.index()) {
                return Some(vertex);
            }
        }
        None
    }

    pub fn vertex(&self, index: usize) -> Option<&ResidualVertex> {
        if !self.has_vertex(index) {
            return None;
        }
        Some(&self.vertices[index])
    }

    pub fn vertex_mut(&mut self, index: usize) -> Option<&mut ResidualVertex> {
        if !self.has_vertex(index) {
            return None;
        }
        Some(&mut self.vertices[index])
    }

    #[allow(dead_code)]
    pub fn remove_vertex(&mut self, vertex: usize) {
        if vertex >= self.vertices.len() {
            return;
        }
        self.remove_edges_of_vertex(vertex);
        self.vertices[vertex].set_active(false);
    }

    fn add_non_active_vertex(&mut self) {
        self.vertices
            .push(ResidualVertex::new_non_active(self.vertices.len()));
    }

    pub fn increase_edge_capacity(&mut self, from: usize, to: usize) {
        if let Some(vertex) = self.vertex_mut(from) {
            vertex.increase_edge_capacity(to);
        }
    }

    pub fn decrease_edge_capacity(&mut self, from: usize, to: usize) -> bool {
        if let Some(vertex) = self.vertex_mut(from) {
            return vertex.decrease_edge_capacity(to);
        }
        false
    }

    #[allow(dead_code)]
    pub fn edge_capacity(&self, from: usize, to: usize) -> usize {
        if let Some(vertex) = self.vertex(from) {
            return vertex.edge_capacity(to);
        }
        0
    }
}

/// Vertices
pub struct Vertices<'a> {
    vertices: slice::Iter<'a, ResidualVertex>,
}

impl<'a> Vertices<'a> {
    pub fn new(vertices: slice::Iter<'a, ResidualVertex>) -> Self {
        Vertices { vertices }
    }
}

impl<'a> Iterator for Vertices<'a> {
    type Item = &'a ResidualVertex;

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
    vertices: &'a Vec<ResidualVertex>,
    position: (usize, usize),
}

impl<'a> Edges<'a> {
    pub fn new(vertices: &'a Vec<ResidualVertex>) -> Self {
        Edges {
            vertices,
            position: (0, 0),
        }
    }
}

impl<'a> Iterator for Edges<'a> {
    type Item = &'a DirectedFlowEdge;
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

impl fmt::Display for ResidualGraph {
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
