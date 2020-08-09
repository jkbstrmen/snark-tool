use crate::graph::edge::{Edge, EdgeConstructor};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct UndirectedEdge {
    from: usize,
    to: usize,
    color: u8,
}

impl Edge for UndirectedEdge {
    fn from(&self) -> usize {
        self.from
    }

    fn to(&self) -> usize {
        self.to
    }

    fn color(&self) -> u8 {
        unimplemented!()
    }

    fn set_color(&mut self, color: u8) {
        self.color = color;
    }
}

impl EdgeConstructor for UndirectedEdge {
    fn new(from: usize, to: usize) -> Self {
        if from > to {
            return UndirectedEdge {
                from: to,
                to: from,
                color: 0,
            };
        }
        UndirectedEdge { from, to, color: 0 }
    }

    fn new_with_colour(from: usize, to: usize, colour: u8) -> Self {
        let mut edge = UndirectedEdge::new(from, to);
        edge.set_color(colour);
        edge
    }
}

impl<'a> Edge for &'a UndirectedEdge {
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
