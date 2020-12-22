use crate::graph::edge::{Edge, EdgeConstructor};
use crate::graph::undirected::edge::UndirectedEdge;
use crate::graph::vertex::{Vertex, VertexConstructor};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct VertexWithNeighbors {
    index: usize,
    neighbors: Vec<Neighbor>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Neighbor {
    index: usize,
    colour: usize,
}

impl Neighbor {
    pub fn new(index: usize) -> Self {
        Neighbor { index, colour: 0 }
    }

    pub fn new_with_colour(index: usize, colour: usize) -> Self {
        Neighbor { index, colour }
    }

    pub fn index(&self) -> usize {
        self.index
    }
    pub fn colour(&self) -> usize {
        self.colour
    }
}

impl Vertex for VertexWithNeighbors {
    fn index(&self) -> usize {
        self.index
    }
}

impl VertexConstructor for VertexWithNeighbors {
    fn new(index: usize) -> Self {
        VertexWithNeighbors {
            index,
            neighbors: vec![],
        }
    }
}

impl VertexWithNeighbors {
    pub fn neighbors(&self) -> &Vec<Neighbor> {
        &self.neighbors
    }

    pub fn set_neighbors(&mut self, neighbors: Vec<Neighbor>) {
        self.neighbors = neighbors
    }

    pub fn add_neighbor(&mut self, neighbor: usize) {
        self.neighbors.push(Neighbor::new(neighbor));
    }

    pub fn add_neighbor_with_colour(&mut self, neighbor: usize, colour: usize) {
        self.neighbors
            .push(Neighbor::new_with_colour(neighbor, colour));
    }

    pub fn remove_neighbor(&mut self, neighbor_to_remove: usize) {
        self.neighbors.retain(|neighbor| {
            neighbor.index != neighbor_to_remove
        });
    }
}
