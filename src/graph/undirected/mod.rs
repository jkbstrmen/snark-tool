use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;

pub mod edge;
pub mod graph;
pub mod vertex;

pub trait UndirectedGraph: Graph<E = UndirectedEdge> {}
