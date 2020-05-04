use crate::graph::vertex::Vertex;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct SimpleVertex {
    index: usize,
}

impl Vertex for SimpleVertex {
    fn new(index: usize) -> Self {
        SimpleVertex { index }
    }
    fn index(&self) -> usize {
        self.index
    }
}
