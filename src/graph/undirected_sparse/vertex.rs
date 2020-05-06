use std::slice::Iter;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct VertexWithNeighbors {
    index: usize,
    pub neighbors: Vec<Neighbor>,
}

impl VertexWithNeighbors {
    pub fn new(index: usize) -> Self {
        VertexWithNeighbors {
            index,
            neighbors: vec![],
        }
    }
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn add_neighbor(&mut self, index: usize, colour: u8) {
        self.neighbors.push(Neighbor::new(index, colour));
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Neighbor {
    index: usize,
    colour: u8,
}

impl Neighbor {
    pub fn new(index: usize, colour: u8) -> Self {
        Neighbor { index, colour }
    }

    pub fn index(&self) -> usize {
        self.index
    }
    pub fn colour(&self) -> u8 {
        self.colour
    }
}
