pub trait Vertex {
    fn index(&self) -> usize;
}

pub trait VertexConstructor {
    fn new(index: usize) -> Self;
}
