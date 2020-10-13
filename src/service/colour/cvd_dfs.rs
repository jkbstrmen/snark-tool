use crate::graph::graph::Graph;
use crate::service::colour::dfs_improved::DFSColourizer;
use crate::service::colour::colouriser::Colourizer;
use crate::service::colour::cvd;

// Colorizer for (sub)cubic graphs only
pub struct CvdDfsColourizer {}

impl Colourizer for CvdDfsColourizer {
    fn is_colorable<G>(graph: &G) -> bool
    where
        G: Graph,
    {
        let result_cvd = cvd::is_colorable(graph);
        if result_cvd.is_none() {
            let result = DFSColourizer::is_colorable(graph);
            return result;
        }
        result_cvd.unwrap()
    }

    fn new() -> Self {
        CvdDfsColourizer {}
    }
}
