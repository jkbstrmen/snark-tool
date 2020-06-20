use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;

pub trait Colourizer {
    fn is_colorable<G: Graph>(graph: &G) -> bool;
    fn new() -> Self;
}
