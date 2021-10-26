use crate::graph::edge::Edge;
use crate::graph::vertex::Vertex;

pub trait Graph {
    type V: Vertex;
    type E: Edge;

    fn size(&self) -> usize;
    fn has_edge(&self, from: usize, to: usize) -> bool;

    fn add_vertex(&mut self);
    fn add_edge(&mut self, from: usize, to: usize);
    fn remove_edge(&mut self, from: usize, to: usize);

    fn remove_edges_of_vertex(&mut self, vertex_index: usize);
    fn remove_vertex(&mut self, vertex_index: usize);
    fn vertices<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::V> + 'a>;

    fn edges<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::E> + 'a>;
    fn edges_of_vertex<'a>(&'a self, vertex: usize) -> Box<dyn Iterator<Item = &'a Self::E> + 'a>;
    fn neighbors_of_vertex(&self, vertex: usize) -> Vec<usize>;
}

pub trait GraphConstructor {
    // CONSTRUCTORS
    fn new() -> Self;
    fn with_capacity(vertices: usize, edges: usize) -> Self;
    fn with_vertices_capacity(vertices: usize) -> Self;
}
