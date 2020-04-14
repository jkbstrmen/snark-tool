use crate::graph::traits::edge::Edge;

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl UndirectedEdge {
    pub fn new(from: usize, to: usize) -> Self {
        if from > to {
            return UndirectedEdge {
                from: to,
                to: from,
                color: 0,
            };
        }
        UndirectedEdge { from, to, color: 0 }
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
