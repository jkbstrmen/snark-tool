use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;

pub trait Colourizer {
    fn is_colorable<G, V, E>(graph: &G) -> bool
    where
        G: Graph<V, E>,
        V: Vertex,
        E: Edge;

    fn new() -> Self;

    // temp
    fn is_colorable_with_counter<G, V, E>(graph: &G, counter: &mut usize) -> bool
    where
        G: Graph<V, E>,
        V: Vertex,
        E: Edge;
}
