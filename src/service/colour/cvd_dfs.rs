use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::vertex::Vertex;
use crate::service::colour::bfs::BFSColourizer;
use crate::service::colour::colouriser::Colourizer;
use crate::service::colour::cvd;

// Colorizer for (sub)cubic graphs only
pub struct CvdDfsColourizer {}

impl Colourizer for CvdDfsColourizer {
    fn is_colorable<G, V, E>(graph: &G) -> bool
    where
        G: Graph<V, E>,
        V: Vertex,
        E: Edge,
    {
        let result_cvd = cvd::is_colorable(graph);
        if result_cvd.is_none() {
            println!("none");
            return false;


            let result = BFSColourizer::is_colorable(graph);

            // if result == true {
            //     println!("here");
            // }

            return result;
        }
        result_cvd.unwrap()
    }

    fn new() -> Self {
        CvdDfsColourizer{}
    }


    // todo - temp
    fn is_colorable_with_counter<G, V, E>(graph: &G, counter: &mut usize) -> bool where
        G: Graph<V, E>,
        V: Vertex,
        E: Edge {
        let result_cvd = cvd::is_colorable(graph);
        if result_cvd.is_none() {
            let result = BFSColourizer::is_colorable(graph);

            if result == true {
                *counter += 1;
                // println!("here");
            }

            return result;
        }
        result_cvd.unwrap()
    }
}
