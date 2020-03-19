use serde::export::fmt::Error;
use serde::export::Formatter;
use std::slice::Iter;
use std::{fmt, slice};

pub trait Graph<V = SimpleVertex, E = UndirectedEdge>
where
    E: Edge,
{
    // fn add_edge(&mut self, edge: E) ;
    fn add_edge(&mut self, from: usize, to: usize);

    // temp
    // fn from_str(source: &str) -> Self;

    // size
    // has_edge
    // edge_iter = edge_iterator
    // vertex_iter = vertex_iterator
    fn add_vertex(&mut self);
    // remove_edge
    // remove_vertex
    // edges = edges of vertex - as iterator?
    fn edges<I>(&self, vertex: usize) -> I
    where
        I: Iterator;
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

    // CONSTRUCTORS
}

// Edges - iterator over the edges ...
pub struct UndirectedEdges<'a> {
    // edges: &'a Vec<UndirectedEdge>,
    vertex: usize,
    iter: slice::Iter<'a, UndirectedEdge>, // next: &'a UndirectedEdge
}

impl<'a> UndirectedEdges<'a> {
    pub fn new(vertex: usize, iter: slice::Iter<'a, UndirectedEdge>) -> Self {
        // let iter = edges.iter();
        //
        // let next = match edges.get(0) {
        //     None => {Option::None},
        //     Some(edge) => {Option::Some(edge.clone())},
        // };
        // UndirectedEdges { edges, iter: edges.iter() }
        UndirectedEdges { vertex, iter }
        // unimplemented!()
    }
}

impl<'a> Iterator for UndirectedEdges<'a> {
    type Item = &'a UndirectedEdge;

    fn next(&mut self) -> Option<Self::Item> {
        // match self.iter.next() {
        //     None => {Option::None},
        //     Some(edge) => {Option::Some(edge.clone())},
        // }
        // self.iter.next()
        let mut next_opt = self.iter.next();
        // if next_opt.is_some() {
        //     let next = next_opt.unwrap();
        //     if next.from == self.vertex || next.to == self.vertex {
        //         return Some(next)
        //     }
        // }

        while next_opt.is_some() {
            let next = next_opt.unwrap();
            if next.from == self.vertex || next.to == self.vertex {
                return Some(next)
            }
            next_opt = self.iter.next();
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct UndirectedEdge {
    from: usize,
    to: usize,
}

impl Edge for UndirectedEdge {
    fn new(from: usize, to: usize) -> Self {
        if from > to {
            return UndirectedEdge { from: to, to: from };
        }
        UndirectedEdge { from, to }
    }
    fn from(&self) -> usize {
        self.from
    }

    fn to(&self) -> usize {
        self.to
    }
}

pub trait Vertex {
    fn new(index: usize) -> Self;
    fn index(&self) -> usize;
    // index
    // weight

    // CONSTRUCTORS
}

#[derive(Debug)]
pub struct SimpleVertex {
    index: usize,
}

impl Vertex for SimpleVertex {
    fn new(index: usize) -> Self {
        SimpleVertex { index }
    }
    fn index(&self) -> usize {
        self.index
    }
}

#[derive(Debug)]
pub struct SimpleGraph {
    // pub graph: String,
    size: usize,
    vertices: Vec<SimpleVertex>,
    edges: Vec<UndirectedEdge>,
}

/// undirected, without loop, without multiple edges
impl Graph for SimpleGraph {
    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push(UndirectedEdge::new(from, to));
        self.edges.sort_by(|a, b| {
            if a.from == b.from {
                return a.to.cmp(&b.to);
            }
            a.from.cmp(&b.from)
        });
    }

    fn add_vertex(&mut self) {
        self.vertices.push(SimpleVertex::new(self.vertices.len()));
    }

    fn edges<I>(&self, vertex: usize) -> I
    where
        I: Iterator,
    {
        unimplemented!()
    }

    fn with_capacity(vertices: usize, edges: usize) -> Self {
        SimpleGraph {
            size: 0,
            vertices: Vec::with_capacity(vertices),
            edges: Vec::with_capacity(edges),
        }
    }

    // fn from_str(source: &str) -> Self {
    //     SimpleGraph {
    //         graph: String::from(source),
    //     }
    // }
}

impl fmt::Display for SimpleGraph {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for vertex in &self.vertices {
            write!(f, "{}: ", vertex.index());

            writeln!(f);
        }

        // for edge in &self.edges {
        //
        // }

        Ok(())
    }
}
