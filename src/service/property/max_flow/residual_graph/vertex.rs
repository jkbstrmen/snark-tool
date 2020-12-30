use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::vertex::{Vertex, VertexConstructor};
use crate::service::property::max_flow::residual_graph::edge::DirectedFlowEdge;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct ResidualVertex {
    index: usize,
    active: bool,
    pub edges: Vec<DirectedFlowEdge>,
}

impl Vertex for ResidualVertex {
    fn index(&self) -> usize {
        self.index
    }
}

impl VertexConstructor for ResidualVertex {
    fn new(index: usize) -> Self {
        ResidualVertex {
            index,
            active: true,
            edges: vec![],
        }
    }
}

impl ResidualVertex {
    pub fn new_non_active(index: usize) -> Self {
        ResidualVertex {
            index,
            active: false,
            edges: vec![],
        }
    }

    pub fn add_edge(&mut self, to: usize) {
        // if self.has_neighbor(to) {
        //     self.increase_edge_capacity(to);
        //     return;
        // }
        self.edges
            .push(DirectedFlowEdge::new_with_capacity(self.index, to, 1));
    }

    pub fn neighbors(&self) -> Vec<usize> {
        let mut neighbors = vec![];
        for edge in self.edges.iter() {
            if edge.capacity() > 0 {
                neighbors.push(edge.to())
            }
        }
        neighbors
    }

    pub fn has_neighbor(&self, neighbor_index: usize) -> bool {
        for neighbor in self.neighbors() {
            if neighbor == neighbor_index {
                return true;
            }
        }
        false
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn edge_capacity(&self, to: usize) -> usize {
        let mut capacity = 0;
        for edge in self.edges.iter() {
            if edge.to() == to {
                capacity += edge.capacity();
            }
        }
        capacity
    }

    pub fn increase_edge_capacity(&mut self, to: usize) {
        for edge in self.edges.iter_mut() {
            if edge.to() == to {
                edge.set_capacity(edge.capacity() + 1);
            }
        }
    }

    ///
    /// decreases edge capacity by 1 if edge is present and capacity is bigger than 0
    /// returns true if edge capacity was decreased, false if vertex does not have edge or its capacity is already 0
    ///
    pub fn decrease_edge_capacity(&mut self, to: usize) -> bool {
        for edge in self.edges.iter_mut() {
            if edge.to() == to {
                if edge.capacity() > 0 {
                    edge.set_capacity(edge.capacity() - 1);
                    return true;
                }
            }
        }
        false
    }
}
