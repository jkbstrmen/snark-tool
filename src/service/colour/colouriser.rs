use crate::graph::graph::Graph;

pub trait Colouriser: Clone {
    fn is_colorable<G: Graph>(graph: &G) -> bool;
    fn new() -> Self;
}
