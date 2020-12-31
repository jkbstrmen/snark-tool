use crate::graph::edge::{Edge, EdgeConstructor};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct DirectedFlowEdge {
    from: usize,
    to: usize,
    colour: u8,
    capacity: usize,
}

impl Edge for DirectedFlowEdge {
    fn from(&self) -> usize {
        self.from
    }

    fn to(&self) -> usize {
        self.to
    }

    fn color(&self) -> u8 {
        self.colour
    }

    fn set_color(&mut self, color: u8) {
        self.colour = color;
    }
}

impl EdgeConstructor for DirectedFlowEdge {
    fn new(from: usize, to: usize) -> Self {
        DirectedFlowEdge {
            from,
            to,
            colour: 0,
            capacity: 0,
        }
    }

    fn new_with_colour(from: usize, to: usize, colour: u8) -> Self {
        DirectedFlowEdge {
            from,
            to,
            colour,
            capacity: 0,
        }
    }
}

impl<'a> Edge for &'a DirectedFlowEdge {
    fn from(&self) -> usize {
        (*self).from()
    }

    fn to(&self) -> usize {
        (*self).to()
    }

    fn color(&self) -> u8 {
        (*self).color()
    }

    fn set_color(&mut self, _color: u8) {}
}

impl DirectedFlowEdge {
    // pub(crate) fn is_adjacent(&self, other: &DirectedFlowEdge) -> bool {
    //     if self.from() == other.from()
    //         || self.from() == other.to()
    //         || self.to() == other.from()
    //         || self.to() == other.to()
    //     {
    //         return true;
    //     }
    //     false
    // }
    pub fn new_with_capacity(from: usize, to: usize, capacity: usize) -> Self {
        DirectedFlowEdge {
            from,
            to,
            colour: 0,
            capacity,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
    }
}
