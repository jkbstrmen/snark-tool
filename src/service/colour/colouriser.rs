use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;

pub trait Colourizer {
    fn is_colorable<G, V, E>(graph: &G) -> bool
    where
        G: Graph<V, E>,
        V: Vertex,
        E: Edge;
}
