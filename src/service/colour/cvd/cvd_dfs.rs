use crate::graph::graph::Graph;
use crate::service::colour::colouriser::Colouriser;
use crate::service::colour::cvd::cvd;
use crate::service::colour::recursive::dfs_improved::DFSColourizer;

// Colorizer for (sub)cubic graphs only
#[derive(Debug, Clone)]
pub struct CvdDfsColourizer {}

impl Colouriser for CvdDfsColourizer {
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
