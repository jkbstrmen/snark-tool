use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::undirected::simple_graph::SimpleGraph;
use crate::graph::undirected::vertex::SimpleVertex;
use crate::graph::undirected_sparse::graph::SimpleSparseGraph;
use crate::graph::vertex::Vertex;

pub trait TempGraph: Graph {
    type V: Vertex;
    type E: Edge;

    // fn size(&self) -> usize;
    // fn has_edge(&self, from: usize, to: usize) -> bool;
    // // fn edge(&self, from: usize, to: usize) -> Option<E>;
    //
    // // add vertex - with param?
    // fn add_vertex(&mut self);
    // fn add_edge(&mut self, from: usize, to: usize);
    // // remove_edge
    // fn remove_edge(&mut self, from: usize, to: usize);
    // // ??
    // fn remove_edges_of_vertex(&mut self, vertex: usize);
    // // fn remove_edges_of_vertex(self, index: usize) -> Self;
    // // remove_vertex
    // // fn remove_vertex(&mut self, index: usize);
    //
    // // fn vertices(&self) -> Vertices<V>;
    // fn vertices<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::V> + 'a>;
    //
    // // neighbors_of_vertex
    //
    // fn edges<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::E> + 'a>;
    // fn edges_of_vertex<'a>(&'a self, vertex: usize) -> Box<dyn Iterator<Item = &'a Self::E> + 'a>;

    // TODO
    fn neighbors_of_vertex(&self, vertex: usize) -> Vec<usize>; // or Vec<Vertex>
                                                                // fn neighbors_of_vertex(&self) -> Box<dyn Iterator<Item = &'a Self::V> + 'a>; // or Box<dyn Iterator<Item = usize> + 'a>

    // fn edges(&self) -> Edges<E>;
    // fn edges_of_vertex(&self, vertex: usize) -> Edges<E>;

    // edges_count
    // vertices_count
    // update_edge
    // update_vertex

    // ??
    // is_directed
}

impl TempGraph for SimpleGraph {
    type V = SimpleVertex;
    type E = UndirectedEdge;

    fn neighbors_of_vertex(&self, vertex: usize) -> Vec<usize> {
        unimplemented!()
    }
}

impl TempGraph for SimpleSparseGraph {
    type V = SimpleVertex;
    type E = UndirectedEdge;

    fn neighbors_of_vertex(&self, vertex: usize) -> Vec<usize> {
        unimplemented!()
    }
}
