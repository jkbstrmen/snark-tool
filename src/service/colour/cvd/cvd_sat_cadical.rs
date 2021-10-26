use crate::graph::graph::Graph;
use crate::service::colour::colouriser::Colouriser;
use crate::service::colour::cvd::cvd;
use crate::service::colour::sat::sat_cadical::SATColourizerCadical;

// Colorizer for (sub)cubic graphs only
#[derive(Debug, Clone)]
pub struct CvdSatCadicalColourizer {}

impl Colouriser for CvdSatCadicalColourizer {
    fn is_colorable<G>(graph: &G) -> bool
    where
        G: Graph,
    {
        let result_cvd = cvd::is_colorable(graph);
        if result_cvd.is_none() {
            return SATColourizerCadical::is_colorable(graph);
        }
        result_cvd.unwrap()
    }

    fn new() -> Self {
        CvdSatCadicalColourizer {}
    }
}
