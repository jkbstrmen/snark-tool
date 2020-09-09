pub trait Vertex {
    fn index(&self) -> usize;
    // index
    // weight
}

pub trait VertexConstructor {
    fn new(index: usize) -> Self;
}
