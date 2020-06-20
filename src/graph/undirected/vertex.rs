use crate::graph::vertex::{Vertex, VertexConstructor};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct SimpleVertex {
    index: usize,
}

impl Vertex for SimpleVertex {
    fn index(&self) -> usize {
        self.index
    }
}

impl VertexConstructor for SimpleVertex {
    fn new(index: usize) -> Self {
        SimpleVertex { index }
    }
}
