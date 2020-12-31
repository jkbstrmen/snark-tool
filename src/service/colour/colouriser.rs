use crate::graph::graph::Graph;

pub trait Colouriser {
    fn is_colorable<G: Graph>(graph: &G) -> bool;
    fn new() -> Self;
}
