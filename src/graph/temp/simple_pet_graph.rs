use crate::graph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::EdgeRef;
use petgraph::Undirected;
use std::cmp;
use std::cmp::Ordering;

pub struct SimplePetGraph {
    pub graph: StableGraph<u8, u16, Undirected, u8>,
}

// impl Graph for SimplePetGraph {
//     fn add_edge(&mut self, string: &str) {
//         unimplemented!()
//     }
//
//     fn from_str(source: &str) -> Self {
//         unimplemented!()
//     }
// }

impl SimplePetGraph {
    fn edges_vec(&self) {}
}
