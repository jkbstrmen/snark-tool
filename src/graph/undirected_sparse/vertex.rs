use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::vertex::{Vertex, VertexConstructor};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct VertexWithEdges {
    index: usize,
    pub edges: Vec<UndirectedEdge>,
}

impl Vertex for VertexWithEdges {
    fn index(&self) -> usize {
        self.index
    }
}

impl VertexConstructor for VertexWithEdges {
    fn new(index: usize) -> Self {
        VertexWithEdges {
            index,
            edges: vec![],
        }
    }
}

impl VertexWithEdges {
    pub fn add_edge(&mut self, to: usize, colour: u8) {
        self.edges
            .push(UndirectedEdge::new_with_colour(self.index, to, colour));
    }

    pub fn neighbors(&self) -> Vec<usize> {
        let mut neighbors = vec![];
        for edge in self.edges.iter() {
            if edge.from() == self.index {
                neighbors.push(edge.to())
            } else {
                neighbors.push(edge.from());
            }
        }
        neighbors
    }
}
