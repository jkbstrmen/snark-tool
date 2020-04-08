use crate::graph::traits::edge::Edge;
use crate::graph::traits::vertex::Vertex;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::vertex::SimpleVertex;
use std::slice;

pub trait Graph<V = SimpleVertex, E = UndirectedEdge>
where
    V: Vertex,
    E: Edge,
{
    // fn add_edge(&mut self, edge: E) ;
    fn add_edge(&mut self, from: usize, to: usize);

    fn size(&self) -> usize;
    // size
    // has_edge
    fn has_edge(&self, from: usize, to: usize) -> bool;
    fn edge(&self, from: usize, to: usize) -> Option<E>;
    // fn has_edge(&self, edge: &E) -> bool;
    // edge_iter = edge_iterator
    // fn edges(&self);
    // fn edges_mut(&mut self);

    // vertex_iter = vertex_iterator
    fn vertices(&self) -> Vertices<V>;
    fn vertices_mut(&mut self) -> VerticesMut<V>;

    // add vertex - with param?
    fn add_vertex(&mut self);
    // remove_edge
    // remove_vertex
    // edges = edges of vertex - as iterator?
    // fn edges(&self, vertex: usize) -> Eit;
    // fn edges(&self, vertex: usize) -> impl Iterator;
    // fn edges<'a, I>(&'a self, vertex: usize) -> I
    // where
    //     I: Iterator + EdgeIter<'a>;

    fn edges(&self) -> Edges<E>;
    fn edges_mut(&mut self) -> EdgesMut<E>;
    // fn edges(&self, vertex: usize) -> UndirectedEdges;
    fn edges_of_vertex(&self, vertex: usize) -> Edges<E>;

    // vertex = vertex with index
    // vertex_mut

    // edges_count
    // vertices_count
    // update_edge
    // update_vertex

    // CONSTRUCTORS
    fn with_capacity(vertices: usize, edges: usize) -> Self;
    // new
    // with_vertices_capacity
    // with_edes_capacity

    // ??
    // is_directed
}

/// Edges
pub struct Edges<'a, E> {
    vertex: Option<usize>,
    iter: slice::Iter<'a, E>,
}

impl<'a, E> Edges<'a, E> {
    pub fn new(iter: slice::Iter<'a, E>) -> Self {
        Edges { vertex: None, iter }
    }
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

/// EdgesMut
pub struct EdgesMut<'a, E> {
    vertex: Option<usize>,
    iter: slice::IterMut<'a, E>,
}

impl<'a, E> EdgesMut<'a, E> {
    pub fn new(iter: slice::IterMut<'a, E>) -> Self {
        EdgesMut { vertex: None, iter }
    }

    pub fn of_vertex(iter: slice::IterMut<'a, E>, vertex: usize) -> Self {
        EdgesMut {
            vertex: Some(vertex),
            iter,
        }
    }
}

impl<'a, E> Iterator for EdgesMut<'a, E>
where
    E: Edge,
{
    type Item = &'a mut E;
    fn next(&mut self) -> Option<Self::Item> {
        if self.vertex.is_some() {
            let mut edge_opt = self.iter.next();
            while edge_opt.is_some() {
                let edge = edge_opt.unwrap();
                if edge.from() == self.vertex.unwrap() || edge.to() == self.vertex.unwrap() {
                    return Some(edge);
                }
                edge_opt = self.iter.next();
            }
            return None;
        }
        self.iter.next()
    }
}

/// Vertices
pub struct Vertices<'a, V>
where
    V: Vertex,
{
    iter: slice::Iter<'a, V>,
}

impl<'a, V> Vertices<'a, V>
where
    V: Vertex,
{
    pub fn new(iter: slice::Iter<'a, V>) -> Self {
        Vertices { iter }
    }
}

impl<'a, V> Iterator for Vertices<'a, V>
where
    V: Vertex,
{
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// VerticesMut
pub struct VerticesMut<'a, V> {
    iter: slice::IterMut<'a, V>,
}

impl<'a, V> VerticesMut<'a, V> {
    pub fn new(iter: slice::IterMut<'a, V>) -> Self {
        VerticesMut { iter }
    }
}

impl<'a, V> Iterator for VerticesMut<'a, V>
where
    V: Vertex,
{
    type Item = &'a mut V;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
