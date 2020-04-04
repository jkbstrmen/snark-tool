use serde::export::PhantomData;
use std::slice::{Iter, IterMut};
use std::slice;

pub trait Graph<E>
where
    E: Edge
{
    fn add_edge(&mut self, from: usize, to: usize);
    fn edges(&self) -> Edges<E>;
    fn edges_mut(&mut self) -> EdgesMut<E>;
    fn edges_of_vertex(&self, vertex: usize) -> Edges<E>;
}

pub trait Edge {
    fn from(&self) -> usize;
    fn to(&self) -> usize;

    fn color(&self) -> u8;
    fn set_color(&mut self, color: u8);
}

pub struct Edges<'a, E> {
    vertex: Option<usize>,
    iter: slice::Iter<'a, E>
}

impl<'a, E> Edges<'a, E> {
    pub fn new(iter: slice::Iter<'a, E>) -> Self {
        Edges{
            vertex: None,
            iter
        }
    }

    pub fn of_vertex(iter: slice::Iter<'a, E>, vertex: usize) -> Self {
        Edges{
            vertex: Some(vertex),
            iter
        }
    }
}

impl<'a, E> Iterator for Edges<'a, E> where E: Edge {
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

pub struct EdgesMut<'a, E> {
    vertex: Option<usize>,
    iter: slice::IterMut<'a, E>
}

impl<'a, E> EdgesMut<'a, E> {
    pub fn new(iter: slice::IterMut<'a, E>) -> Self {
        EdgesMut{
            vertex: None,
            iter
        }
    }

    pub fn of_vertex(iter: slice::IterMut<'a, E>, vertex: usize) -> Self {
        EdgesMut{
            vertex: Some(vertex),
            iter
        }
    }
}

impl<'a, E> Iterator for EdgesMut<'a, E> where E: Edge {
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

// pub trait Edges<'a, E: 'a> {
//     fn iter(&self) -> Iterator<Item = &'a E>;
// }

// pub trait EdgesMut<'a, E: 'a>: Iterator<Item = &'a mut E> {}

//
//  IMPL
//

#[derive(Debug)]
pub struct EdgeImpl {
    from: usize,
    to: usize,
    color: u8,
}

impl Edge for EdgeImpl {
    fn from(&self) -> usize {
        self.from
    }

    fn to(&self) -> usize {
        self.to
    }

    fn color(&self) -> u8 {
        self.color
    }

    fn set_color(&mut self, color: u8) {
        self.color = color;
    }
}
//
// pub struct EdgesImpl<'a> {
//     edges: Iter<'a, EdgeImpl>,
// }
//
// // impl<'a> Iterator for EdgesImpl<'a> {
// //     type Item = &'a EdgeImpl;
// //
// //     fn next(&mut self) -> Option<Self::Item> {
// //         self.edges.next()
// //     }
// // }
//
// impl<'a> Edges<'a, EdgeImpl> for EdgesImpl<'a> {
//     fn iter(&self) -> Iter<EdgeImpl> {
//         unimplemented!()
//     }
// }
//
// impl<'a> EdgesImpl<'a> {
//     pub fn new(edges: &'a Vec<EdgeImpl>) -> Self {
//         EdgesImpl {
//             edges: edges.iter(),
//         }
//     }
// }
//
// pub struct EdgesMutImpl<'a> {
//     edges: IterMut<'a, EdgeImpl>,
// }
//
// impl<'a> Iterator for EdgesMutImpl<'a> {
//     type Item = &'a mut EdgeImpl;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.edges.next()
//     }
// }
//
// impl<'a> EdgesMut<'a, EdgeImpl> for EdgesMutImpl<'a> {}
//
// impl<'a> EdgesMutImpl<'a> {
//     pub fn new(edges: &'a mut Vec<EdgeImpl>) -> Self {
//         EdgesMutImpl {
//             edges: edges.iter_mut(),
//         }
//     }
// }
//
#[derive(Debug)]
pub struct GraphImpl {
    edges: Vec<EdgeImpl>,
}

impl GraphImpl {
    pub fn new() -> Self {
        GraphImpl { edges: vec![] }
    }
}

impl Graph<EdgeImpl> for GraphImpl {
    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push(EdgeImpl{
            from,
            to,
            color: 0
        })
    }

    fn edges(&self) -> Edges<EdgeImpl> {
        Edges::new(self.edges.iter())
    }

    fn edges_mut(&mut self) -> EdgesMut<EdgeImpl> {
        EdgesMut::new(self.edges.iter_mut())
    }

    fn edges_of_vertex(&self, vertex: usize) -> Edges<EdgeImpl> {
        Edges::of_vertex(self.edges.iter(), vertex)
    }


    // fn add_edge(&mut self, from: usize, to: usize) {
    //     self.edges.push(EdgeImpl { from, to, color: 0 })
    // }
    //
    // fn edges(&'a self) -> EdgesImpl {
    //     EdgesImpl::new(&self.edges)
    //     // EdgesImpl{
    //     //     edges: self.edges.iter()
    //     // }
    // }
    //
    // fn edges_mut(&'a mut self) -> EdgesMutImpl<'a> {
    //     EdgesMutImpl::new(&mut self.edges)
    // }
}
