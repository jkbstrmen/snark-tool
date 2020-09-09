use crate::graph::edge::Edge;
use crate::graph::vertex::Vertex;

pub trait Graph {
    type V: Vertex;
    type E: Edge;

    fn size(&self) -> usize;
    fn has_edge(&self, from: usize, to: usize) -> bool;
    // fn edge(&self, from: usize, to: usize) -> Option<E>;

    // add vertex - with param?
    fn add_vertex(&mut self);
    fn add_edge(&mut self, from: usize, to: usize);
    // remove_edge
    fn remove_edge(&mut self, from: usize, to: usize);
    // ??
    fn remove_edges_of_vertex(&mut self, vertex: usize);
    // fn remove_edges_of_vertex(self, index: usize) -> Self;
    // remove_vertex
    // fn remove_vertex(&mut self, index: usize);

    // fn vertices(&self) -> Vertices<V>;
    fn vertices<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::V> + 'a>;

    fn edges<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::E> + 'a>;
    fn edges_of_vertex<'a>(&'a self, vertex: usize) -> Box<dyn Iterator<Item = &'a Self::E> + 'a>;

    // fn edges(&self) -> Edges<E>;
    // fn edges_of_vertex(&self, vertex: usize) -> Edges<E>;

    // edges_count
    // vertices_count
    // update_edge
    // update_vertex

    // ??
    // is_directed
}

pub trait GraphConstructor {
    // CONSTRUCTORS
    fn new() -> Self;
    fn with_capacity(vertices: usize, edges: usize) -> Self;
    fn with_vertices_capacity(vertices: usize) -> Self;
    // with_edes_capacity
}
