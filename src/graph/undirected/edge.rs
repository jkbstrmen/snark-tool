use crate::graph::traits::edge::Edge;

#[derive(Debug, Clone)]
pub struct UndirectedEdge {
    from: usize,
    to: usize,
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
        unimplemented!()
    }
}

impl UndirectedEdge {
    pub fn new(from: usize, to: usize) -> Self {
        if from > to {
            return UndirectedEdge { from: to, to: from };
        }
        UndirectedEdge { from, to }
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
        unimplemented!()
    }

    fn set_color(&mut self, color: u8) {
        unimplemented!()
    }
}
