use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::graph::{Graph, GraphConstructor};
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected_sparse::vertex::VertexWithEdges;
use crate::graph::undirected_sparse_2::vertex::VertexWithNeighbors;
use crate::graph::vertex::{Vertex, VertexConstructor};
use std::{fmt, cmp};

///
/// uses Vec of neighbors for each vertex
///
#[derive(Debug, Clone)]
pub struct SimpleSparseGraph {
    pub size: usize,
    pub vertices: Vec<VertexWithNeighbors>,
}

impl Graph for SimpleSparseGraph {
    type V = VertexWithNeighbors;
    type E = UndirectedEdge;

    fn size(&self) -> usize {
        self.vertices.len()
    }

    fn has_edge(&self, from: usize, to: usize) -> bool {
        if from >= self.vertices.len() || to >= self.vertices.len() {
            return false;
        }
        let from_vertex = &self.vertices[from];
        for neighbor in from_vertex.neighbors() {
            if neighbor.index() == to {
                return true;
            }
        }
        false
    }

    fn add_vertex(&mut self) {
        self.vertices
            .push(VertexWithNeighbors::new(self.vertices.len()));
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        if self.has_edge(from, to) {
            return;
        }
        while self.vertices.len() <= cmp::max(from, to) {
            self.add_vertex();
        }
        let from_vertex = &mut self.vertices[from];
        from_vertex.add_neighbor(to);
        let to_vertex = &mut self.vertices[to];
        to_vertex.add_neighbor(from);
    }

    fn remove_edge(&mut self, from: usize, to: usize) {
        if from >= self.size() || to >= self.size() {
            return;
        }
        let from_vertex = &mut self.vertices[from];
        from_vertex.remove_neighbor(to);
        let to_vertex = &mut self.vertices[to];
        to_vertex.remove_neighbor(from);
    }

    fn remove_edges_of_vertex(&mut self, vertex: usize) {
        let neighbors = self.vertex(vertex).neighbors().clone();
        for neighbor in neighbors {
            self.vertex_mut(neighbor.index()).remove_neighbor(vertex);
        }
        self.vertices[vertex].set_neighbors(vec![]);
    }

    fn vertices<'a>(&'a self) -> Box<dyn Iterator<Item = &'a VertexWithNeighbors> + 'a> {
        Box::new(self.vertices.iter())
    }

    fn edges<'a>(&'a self) -> Box<dyn Iterator<Item = &'a UndirectedEdge> + 'a> {
        Box::new(Edges::new(&self.vertices))
    }

    fn edges_of_vertex<'a>(
        &'a self,
        vertex: usize,
    ) -> Box<dyn Iterator<Item = &'a UndirectedEdge> + 'a> {
        // Box::new(self.vertices[vertex].edges.iter())
        unimplemented!()
    }
}

impl GraphConstructor for SimpleSparseGraph {
    fn new() -> Self {
        Self::with_vertices_capacity(20)
    }

    fn with_capacity(vertices: usize, _edges: usize) -> Self {
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

    pub fn vertex(&self, index: usize) -> &VertexWithNeighbors {
        &self.vertices[index]
    }

    pub fn vertex_mut(&mut self, index: usize) -> &mut VertexWithNeighbors {
        &mut self.vertices[index]
    }
}

/// Edges
pub struct Edges<'a> {
    vertices: &'a Vec<VertexWithNeighbors>,
    position: (usize, usize),
    edges: Vec<UndirectedEdge>,
}

impl<'a> Edges<'a> {
    pub fn new(vertices: &'a Vec<VertexWithNeighbors>) -> Self {
        Edges {
            vertices,
            position: (0, 0),
            edges: vec![]
        }
    }
}

impl<'a> Iterator for Edges<'a> {
    type Item = &'a UndirectedEdge;
    fn next(&mut self) -> Option<Self::Item> {
        // if self.vertices.len() <= self.position.0 {
        //     return None;
        // }
        //
        // let vertex = &self.vertices[self.position.0];
        // // go to next vertex if no neighbor left
        // if vertex.neighbors().len() <= self.position.1 {
        //     self.position.0 += 1;
        //     self.position.1 = 0;
        //     return self.next();
        // }
        //
        // let neighbor = vertex.neighbors().get(self.position.1).unwrap();
        // if neighbor.index() < vertex.index() {
        //     self.position.1 += 1;
        //     return self.next();
        // }
        // self.position.1 += 1;
        //
        // let edge = UndirectedEdge::new(vertex.index(), neighbor.index());
        // self.edges.push(edge);
        // // let edge: &'a UndirectedEdge = &self.edges[self.edges.len()];
        // let edge = &self.edges[self.edges.len()];
        // Some(edge)

        unimplemented!()
    }
}

impl fmt::Display for SimpleSparseGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // for vertex in &self.vertices {
        //     write!(f, "{}: ", vertex.index())?;
        //     let mut separ = String::from("");
        //     for edge in vertex.edges.iter() {
        //         let neighbor = if edge.from() == vertex.index() {
        //             edge.to()
        //         } else {
        //             edge.from()
        //         };
        //         write!(f, "{}{}", separ, neighbor)?;
        //         separ = String::from(", ");
        //     }
        //
        //     writeln!(f)?;
        // }
        // Ok(())

        unimplemented!()
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
