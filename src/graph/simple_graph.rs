use crate::graph::graph;
use crate::graph::graph::{Edge, Edges, Graph, Vertex, Vertices};
use serde::export::fmt::Error;
use serde::export::Formatter;
use std::collections::HashMap;
use std::fmt;
use std::iter::Map;

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

pub struct EdgesOfVertex<'a, E> {
    vertex: usize,
    edges: &'a Vec<E>,
    current_index: usize,
}

impl<'a, E> EdgesOfVertex<'a, E> {
    pub fn new(vertex: usize, edges: &'a Vec<E>) -> Self {
        Self {
            vertex,
            edges,
            current_index: 0,
        }
    }
}

impl<'a> Iterator for EdgesOfVertex<'a, UndirectedEdge> {
    type Item = &'a UndirectedEdge;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_index < self.edges.len() {
            let next_opt = self.edges.get(self.current_index);
            if next_opt.is_some() {
                let next = next_opt.unwrap();
                if next.from == self.vertex || next.to == self.vertex {
                    self.current_index += 1;
                    return Some(next);
                }
            }
            self.current_index += 1;
        }
        None
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
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
    size: usize,
    vertices: Vec<SimpleVertex>,
    edges: Vec<UndirectedEdge>,
    // edges_2: HashMap<SimpleVertex, Vec<UndirectedEdge>>
    // TODO impl hash map for vert->edges ?? - for fast edge retrieval (edges of vertex)
}

/// undirected, without loop, without multiple edges
impl Graph for SimpleGraph {
    fn add_edge(&mut self, from: usize, to: usize) {
        let edge = UndirectedEdge::new(from, to);
        if self.has_edge(&edge) {
            return;
        }
        self.edges.push(edge.clone());
        self.edges.sort_by(|a, b| {
            if a.from == b.from {
                return a.to.cmp(&b.to);
            }
            a.from.cmp(&b.from)
        });
        while self.vertices.len() <= edge.to {
            self.add_vertex();
        }
    }

    fn size(&self) -> usize {
        self.vertices.len()
    }

    fn has_edge(&self, edge: &UndirectedEdge) -> bool {
        for edge_ex in &self.edges {
            if edge_ex.from == edge.from && edge_ex.to == edge.to {
                return true;
            }
        }
        false
    }

    fn vertices(&self) -> Vertices<SimpleVertex> {
        Vertices::new(&self.vertices)
    }

    fn add_vertex(&mut self) {
        self.vertices.push(SimpleVertex::new(self.vertices.len()));
    }

    /*fn edges(&self, vertex: usize) -> graph::Edges<UndirectedEdge> {
        unimplemented!()
    }*/

    fn edges_of_vertex(&self, vertex: usize) -> EdgesOfVertex<UndirectedEdge> {
        EdgesOfVertex::new(vertex, &self.edges)
    }

    fn with_capacity(vertices: usize, edges: usize) -> Self {
        SimpleGraph {
            size: 0,
            vertices: Vec::with_capacity(vertices),
            edges: Vec::with_capacity(edges),
            // edges_2: HashMap::new()
        }
    }
}

impl fmt::Display for SimpleGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for vertex in &self.vertices {
            write!(f, "{}: ", vertex.index());
            let mut separ = String::from("");
            for edges_of_vertex in self.edges_of_vertex(vertex.index()) {
                if edges_of_vertex.from == vertex.index {
                    write!(f, "{}{}", separ, edges_of_vertex.to);
                } else {
                    write!(f, "{}{}", separ, edges_of_vertex.from);
                }
                separ = String::from(", ");
            }
            writeln!(f);
        }
        Ok(())
    }
}

// impl fmt::Debug for HashMap<SimpleVertex, Vec<UndirectedEdge>> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         unimplemented!()
//     }
// }

//
//
//
//
//
// pub struct UndirectedEdges<'a> {
//     vertex: usize,
//     iter: slice::Iter<'a, UndirectedEdge>, // next: &'a UndirectedEdge
// }
//
// pub trait EdgeIter<'a> {
//     fn new(vertex: usize, iter: slice::Iter<'a, UndirectedEdge>) -> Self;
// }
//
// impl<'a> EdgeIter<'a> for UndirectedEdges<'a> {
//     fn new(vertex: usize, iter: Iter<'_, UndirectedEdge>) -> Self {
//         Self{ vertex, iter }
//     }
// }
// impl<'a> Iterator for UndirectedEdges<'a> {
//     type Item = &'a UndirectedEdge;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         // match self.iter.next() {
//         //     None => {Option::None},
//         //     Some(edge) => {Option::Some(edge.clone())},
//         // }
//         // self.iter.next()
//         let mut next_opt = self.iter.next();
//         // if next_opt.is_some() {
//         //     let next = next_opt.unwrap();
//         //     if next.from == self.vertex || next.to == self.vertex {
//         //         return Some(next)
//         //     }
//         // }
//
//         while next_opt.is_some() {
//             let next = next_opt.unwrap();
//             if next.from == self.vertex || next.to == self.vertex {
//                 return Some(next)
//             }
//             next_opt = self.iter.next();
//         }
//         None
//     }
// }

// pub struct UndirectedEdges<'a> {
//     vertex: usize,
//     edges: &'a Vec<UndirectedEdge>
// }
//
// impl<'a> UndirectedEdges<'a> {
//     pub fn new(vertex: usize, edges: &'a Vec<UndirectedEdge>) -> Self {
//         UndirectedEdges{ vertex, edges }
//     }
// }
//
// impl<'a> Iterator for UndirectedEdges<'a> {
//     type Item = &'a UndirectedEdge;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         Some(&UndirectedEdge{ from: 0, to: 0 })
//     }
// }
