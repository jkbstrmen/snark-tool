use crate::graph::graph::Graph;
use crate::graph::undirected::edge::UndirectedEdge;

pub mod edge;
pub mod multi_graph;
pub mod simple_edge_graph;
pub mod simple_graph;
pub mod vertex;

pub trait UndirectedGraph: Graph<E = UndirectedEdge> {}
