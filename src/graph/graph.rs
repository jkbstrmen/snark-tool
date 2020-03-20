use crate::graph::simple_graph::{EdgesOfVertex, SimpleVertex, UndirectedEdge};
use serde::export::fmt::Error;
use serde::export::{Formatter, PhantomData};
use std::slice::Iter;
use std::{fmt, slice};

// make Eit and Vit - another generic params?
pub trait Graph<V = SimpleVertex, E = UndirectedEdge>
where
    V: Vertex,
    E: Edge,
{
    // fn add_edge(&mut self, edge: E) ;
    fn add_edge(&mut self, from: usize, to: usize);

    // size
    // has_edge
    // edge_iter = edge_iterator
    // fn edges(&self);
    // fn edges_mut(&mut self);

    // vertex_iter = vertex_iterator
    fn vertices(&self) -> Vertices<V>;

    fn add_vertex(&mut self);
    // remove_edge
    // remove_vertex
    // edges = edges of vertex - as iterator?
    // fn edges(&self, vertex: usize) -> impl Iterator;
    // fn edges<'a, I>(&'a self, vertex: usize) -> I
    // where
    //     I: Iterator + EdgeIter<'a>;

    // fn edges(&self, vertex: usize) -> Self::Edges;
    // fn edges(&self, vertex: usize) -> UndirectedEdges;
    fn edges_of_vertex(&self, vertex: usize) -> EdgesOfVertex<E>;

    // vertex = vertex with index
    // vertex_mut

    // edges_count
    // vertices_count
    // update_edge
    // update_vertex

    // CONSTRUCTORS
    fn with_capacity(vertices: usize, edges: usize) -> Self;
    // with_vertices_capacity
    // with_edes_capacity

    // ??
    // is_directed
}

pub trait GraphStructures {
    // ??
    // edges_vec
    // vertices_vec
    // adj_matrix
    // ...
}

pub trait Edge {
    fn new(from: usize, to: usize) -> Self;

    fn from(&self) -> usize;
    fn to(&self) -> usize;
    // colour? or weight
}

pub trait Vertex {
    fn new(index: usize) -> Self;
    fn index(&self) -> usize;
    // index
    // weight
}

pub struct Vertices<'a, V> {
    next: Iter<'a, V>
}

impl<'a> Vertices<'a, SimpleVertex> {
    pub fn new(vertices: &'a Vec<SimpleVertex>) -> Self {
        Vertices{ next: vertices.iter() }
    }
}

impl<'a> Iterator for Vertices<'a, SimpleVertex> {
    type Item = &'a SimpleVertex;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.next()
    }
}
